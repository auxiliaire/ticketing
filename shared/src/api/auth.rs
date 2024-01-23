use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum AuthScheme {
    Basic,
    Bearer,
}
