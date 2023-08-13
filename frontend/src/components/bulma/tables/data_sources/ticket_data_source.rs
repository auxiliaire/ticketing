use crate::{
    components::{
        bulma::tables::{
            composite_cell_data::CompositeCellData,
            table_data_source::{ITableDataSource, TableDataSource},
        },
        priority_tag::PriorityTag,
    },
    Route,
};
use implicit_clone::unsync::{IArray, IString};
use shared::dtos::ticket_dto::{ITicketDto, TicketDto, TicketField, TicketValue};
use std::rc::Rc;
use yew::{classes, html, Callback};
use yew_router::prelude::Link;

pub struct TicketDataSource(ITableDataSource<TicketField, ITicketDto, TicketValue>);

impl From<&Vec<TicketDto>> for TicketDataSource {
    fn from(source: &Vec<TicketDto>) -> Self {
        Self(Rc::new(TableDataSource {
            empty_label: IString::from("No tickets selected for this project"),
            fieldset: IArray::from(vec![
                TicketField::Id,
                TicketField::Title,
                TicketField::Priority,
                TicketField::Status,
            ]),
            data: IArray::from(
                source
                    .iter()
                    .map(|ticket| Rc::new(ticket.clone()))
                    .collect::<Vec<ITicketDto>>(),
            ),
            has_row_head: true,
            headprovider: Some(Callback::from(|field: TicketField| match field {
                TicketField::Id => Some(field.into()),
                TicketField::Title => Some(field.into()),
                TicketField::Priority => Some(field.into()),
                TicketField::Status => Some(field.into()),
                _ => None,
            })),
            cellrenderer: Callback::from(|celldata: CompositeCellData<TicketField, ITicketDto>| {
                match celldata.data.id {
                    Some(id) => match celldata.column {
                        TicketField::Id => Some(html! {
                            {id}
                        }),
                        TicketField::Title => Some(html! {
                            <Link<Route> classes={classes!("column", "is-full", "pl-0", "pt-0", "pb-0")} to={Route::Ticket { id }}>
                                {celldata.data.title.clone()}
                            </Link<Route>>
                        }),
                        TicketField::Priority => Some(html! {
                            <PriorityTag priority={Rc::new(celldata.data.priority.clone())} />
                        }),
                        TicketField::Status => Some(html! {
                            <span class="tag">{celldata.data.status}</span>
                        }),
                        _ => None,
                    },
                    None => None,
                }
            }),
            ..Default::default()
        }))
    }
}

impl From<TicketDataSource> for ITableDataSource<TicketField, ITicketDto, TicketValue> {
    fn from(val: TicketDataSource) -> Self {
        val.0
    }
}
