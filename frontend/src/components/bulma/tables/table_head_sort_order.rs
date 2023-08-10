use implicit_clone::ImplicitClone;
use strum::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString, PartialEq)]
pub enum TableHeadSortOrder {
    #[strum(serialize = "asc")]
    Asc,
    #[strum(serialize = "desc")]
    Desc,
}

impl ImplicitClone for TableHeadSortOrder {}
