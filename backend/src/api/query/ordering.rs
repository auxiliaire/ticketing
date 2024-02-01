use serde::{de::Visitor, Deserialize, Serialize};
use shared::api::helper::empty_string_as_none;

pub struct Order(pub sea_orm::sea_query::types::Order);

impl Serialize for Order {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s_value = match self.0 {
            sea_orm::Order::Asc => String::from("asc"),
            sea_orm::Order::Desc => String::from("desc"),
            sea_orm::Order::Field(_) => String::from("unsupported"),
        };
        serializer.serialize_str(s_value.as_str())
    }
}

struct OrderingVisitor;

impl<'de> Visitor<'de> for OrderingVisitor {
    type Value = Order;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string either \"asc\" or \"desc\"")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_str(v.as_str())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "asc" => Ok(Order(sea_orm::Order::Asc)),
            "desc" => Ok(Order(sea_orm::Order::Desc)),
            u => Err(E::custom(format!("value \"{}\" is not supported", u))),
        }
    }
}

impl<'de> Deserialize<'de> for Order {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(OrderingVisitor)
    }
}

impl Default for Order {
    fn default() -> Self {
        Self(sea_orm::Order::Asc)
    }
}

#[derive(Deserialize)]
pub struct Ordering {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub sort: Option<String>,
    #[serde(default)]
    pub order: Order,
}
