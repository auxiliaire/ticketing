use crate::{
    components::bulma::tables::{
        composite_cell_data::CompositeCellData,
        table_data_source::{ITableDataSource, TableDataSource},
    },
    route::Route,
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
            // Override fieldset:
            fieldset: IArray::from(vec![
                UserField::PublicId,
                UserField::Name,
                UserField::Username,
                UserField::Role,
            ]),
            data: IArray::from(
                source
                    .iter()
                    .map(|ticket| Rc::new(ticket.clone()))
                    .collect::<Vec<IUserDto>>(),
            ),
            has_column_head: true,
            has_row_head: true,
            cellrenderer: Callback::from(|celldata: CompositeCellData<UserField, IUserDto>| {
                match celldata.data.id {
                    Some(id) => {
                        let public_id = celldata.data.public_id.unwrap();
                        let public_id_str = public_id.to_string();
                        let slice = &public_id_str.as_str()[..8];
                        match celldata.column {
                            UserField::Id => Some(html! {
                                { id }
                            }),
                            UserField::PublicId => Some(html! {
                                { slice }
                            }),
                            UserField::Name => Some(html! {
                                <Link<Route> classes={classes!("column", "is-full", "pl-0", "pt-0", "pb-0")} to={Route::User { id: public_id }}>
                                    {celldata.data.name.clone()}
                                </Link<Route>>
                            }),
                            UserField::Username => Some(html! {
                                <a classes={classes!("column", "is-full", "pl-0", "pt-0", "pb-0")} href={format!("mailto:{}", celldata.data.username.clone())}>
                                    {celldata.data.username.to_string().clone()}
                                </a>
                            }),
                            UserField::Role => Some(html! {
                                <span class="tag">{ html! {celldata.data.role.map_or("".to_owned(), |r| format!("{}", r))}}</span>
                            }),
                            // Hide columns:
                            UserField::Action => None,
                            // Add an extra column:
                            // UserField::Action => Some(html! {
                            //    <span class="tag">{ "Action: " }{celldata.data.id}</span>
                            // }),
                        }
                    }
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
