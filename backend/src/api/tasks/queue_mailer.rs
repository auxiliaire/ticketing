use crate::api::config::MailConfig;
use crate::api::consts::STORE_URL;
use crate::api::services::notification_service::NotificationService;
use chrono::Utc;
use fang::async_trait;
use fang::typetag;
use fang::AsyncQueueable;
use fang::AsyncRunnable;
use fang::FangError;
use fang::Scheduled;
use lettre::Message;
use redis::Commands;
use redis::Connection;
use redis::FromRedisValue;
use redis::RedisResult;
use redis::ToRedisArgs;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Clone, Debug, Display, EnumString, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum UpdateMessageKey {
    SubscriberSet,
    Subject,
    Body,
}

impl ToRedisArgs for UpdateMessageKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.to_string().as_bytes())
    }
}

impl From<UpdateMessageKey> for FangError {
    fn from(value: UpdateMessageKey) -> Self {
        FangError {
            description: format!("Key '{}' not found in update message", value),
        }
    }
}

const BATCH_SIZE: u32 = 5;
const TICKET_UPDATES_QUEUE: &str = "ticket:updates";
pub const TICKET_SUBSCRIBER_SET: &str = "ticket:subscribers";
const TICKET_UPDATE_ID: &str = "ticket:update:?";
pub const PROJECT_SUBSCRIBER_SET: &str = "project:subscribers";

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "fang::serde")]
pub struct QueueMailer {}

#[async_trait]
#[typetag::serde]
impl AsyncRunnable for QueueMailer {
    async fn run(&self, _queue: &mut dyn AsyncQueueable) -> Result<(), FangError> {
        tracing::info!("RUNNING QueueMailer");

        // Acquire Redis connection
        let mut con = redis::Client::open(STORE_URL.clone())
            .and_then(|store| store.get_connection())
            .map_err(|e| FangError {
                description: e.to_string(),
            })?;

        // Initialize NotificationService
        let mail_config = MailConfig::default();
        let notification_service = NotificationService::new(mail_config);

        let mut counter = 0_u32;
        // Fetch oldest ticket updates
        while let Ok(update_key) = con.lpop::<&str, String>(TICKET_UPDATES_QUEUE, None) {
            if let Ok(update) = con.hgetall::<String, HashMap<String, String>>(update_key) {
                let single_ticket_subscriber_set = update
                    .get(&UpdateMessageKey::SubscriberSet.to_string())
                    .ok_or(UpdateMessageKey::SubscriberSet)?;
                let subject = update
                    .get(&UpdateMessageKey::Subject.to_string())
                    .ok_or(UpdateMessageKey::Subject)?;
                let body = update
                    .get(&UpdateMessageKey::Body.to_string())
                    .ok_or(UpdateMessageKey::Body)?;
                // Fetch subscribers (killswitch-single ticket intersection)
                let subscribers: Vec<String> = con
                    .sinter(vec![TICKET_SUBSCRIBER_SET, single_ticket_subscriber_set])
                    .map_err(|e| FangError {
                        description: e.to_string(),
                    })?;
                // Notify each subscriber
                subscribers.iter().for_each(|subscriber| {
                    if let Some(message) = subscriber.to_owned().parse().ok().and_then(|mailbox| {
                        Message::builder()
                            .from("System <system@example.com>".parse().unwrap())
                            .to(mailbox)
                            .subject(subject)
                            .body(String::from(body))
                            .ok()
                    }) {
                        let _ = notification_service.send_email(message);
                    }
                });
            }
            if counter > BATCH_SIZE {
                tracing::warn!("Ticket update batch size (< {}) exhausted!", counter);
                break;
            }
            counter += 1;
        }

        Ok(())
    }

    fn cron(&self) -> Option<Scheduled> {
        let expression = "0 */5 * ? * *";
        Some(Scheduled::CronPattern(expression.to_string()))
    }

    fn uniq(&self) -> bool {
        true
    }
}

pub fn ticket_id_subscriber_set(id: u64) -> String {
    TICKET_SUBSCRIBER_SET.replace(':', format!(":{}:", id).as_str())
}

pub fn push_to_queue<RV>(
    con: &mut Connection,
    key: String,
    value: &[(UpdateMessageKey, String)],
) -> RedisResult<RV>
where
    RV: FromRedisValue,
{
    con.hset_multiple::<String, UpdateMessageKey, String, ()>(key.clone(), value)?;
    con.rpush(TICKET_UPDATES_QUEUE, key)
}

pub fn ticket_update_id(id: u64) -> String {
    let ts = Utc::now().timestamp_millis();
    TICKET_UPDATE_ID.replace('?', format!("{}:{}", id, ts).as_str())
}
