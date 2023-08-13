use crate::{
    components::bulma::tables::{
        composite_cell_data::CompositeCellData,
        table_data_source::{ITableDataSource, TableDataSource},
    },
    Route,
};
use implicit_clone::unsync::{IArray, IString};
use shared::dtos::user_dto::{IUserDto, UserDto, UserField, UserValue};
use std::rc::Rc;
use yew::{classes, html, Callback};
use yew_router::prelude::Link;

pub struct UserDataSource(ITableDataSource<UserField, IUserDto, UserValue>);

impl From<&Vec<UserDto>> for UserDataSource {
    fn from(source: &Vec<UserDto>) -> Self {
        Self(Rc::new(TableDataSource {
            empty_label: IString::from("There are no users yet"),
            fieldset: IArray::from(vec![UserField::Id, UserField::Name, UserField::Role]),
            data: IArray::from(
                source
                    .iter()
                    .map(|ticket| Rc::new(ticket.clone()))
                    .collect::<Vec<IUserDto>>(),
            ),
            has_row_head: true,
            headprovider: Some(Callback::from(|field: UserField| match field {
                UserField::Id => Some(field.into()),
                UserField::Name => Some(field.into()),
                UserField::Role => Some(field.into()),
            })),
            cellrenderer: Callback::from(|celldata: CompositeCellData<UserField, IUserDto>| {
                match celldata.data.id {
                    Some(id) => match celldata.column {
                        UserField::Id => Some(html! {
                            {id}
                        }),
                        UserField::Name => Some(html! {
                            <Link<Route> classes={classes!("column", "is-full", "pl-0", "pt-0", "pb-0")} to={Route::User { id }}>
                                {celldata.data.name.clone()}
                            </Link<Route>>
                        }),
                        UserField::Role => Some(html! {
                            <span class="tag">{celldata.data.role}</span>
                        }),
                    },
                    None => None,
                }
            }),
            ..Default::default()
        }))
    }
}

impl From<UserDataSource> for ITableDataSource<UserField, IUserDto, UserValue> {
    fn from(val: UserDataSource) -> Self {
        val.0
    }
}
