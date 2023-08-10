use super::data_source_creator::DataSourceCreator;
use crate::{
    components::bulma::tables::{
        composite_cell_data::CompositeCellData, table_data_source::TableDataSource,
    },
    Route,
};
use implicit_clone::unsync::{IArray, IString};
use shared::dtos::user_dto::{IUserDto, UserDto, UserField};
use std::rc::Rc;
use yew::{classes, html, Callback};
use yew_router::prelude::Link;

pub struct UserDataSource {}

impl DataSourceCreator<&Vec<UserDto>, UserField, IUserDto> for UserDataSource {
    fn create(source: &Vec<UserDto>) -> Rc<TableDataSource<UserField, IUserDto>> {
        Rc::new(TableDataSource {
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
        })
    }
}
