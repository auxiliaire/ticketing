use entity::sea_orm_active_enums::Priority;
use shared::validation::ticket_validation::TicketPriority;
use std::rc::Rc;
use yew::{classes, function_component, html, Html, Properties};

type TagPriority = Rc<TicketPriority>;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub priority: TagPriority,
}

#[function_component(PriorityTag)]
pub fn priority_tag(props: &Props) -> Html {
    let priority_class = match props.priority.0 {
        Priority::Low => "is-info",
        Priority::Normal => "is-success",
        Priority::High => "is-warning",
        Priority::Critical => "is-danger",
    };
    html! {
        <span class={classes!("tag", "is-light", priority_class)}>{ props.priority.to_string() }</span>
    }
}
