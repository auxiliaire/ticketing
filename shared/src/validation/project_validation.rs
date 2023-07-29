use chrono::{DateTime, Days, Utc};
use serde_valid::validation::Error;

pub struct ProjectValidation;

impl ProjectValidation {
    pub fn deadline_validation(deadline: &Option<DateTime<Utc>>) -> Result<(), Error> {
        match deadline.map(|d| Self::is_deadline_valid(&d)) {
            Some(true) => Ok(()),
            _ => Err(Error::Custom(
                "The deadline should be a date in the future.".to_owned(),
            )),
        }
    }

    fn is_deadline_valid(deadline: &DateTime<Utc>) -> bool {
        let tomorrow = Utc::now().checked_add_days(Days::new(1)).unwrap();
        deadline >= &tomorrow
    }
}
