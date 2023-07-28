use serde::Deserialize;

#[derive(Deserialize)]
pub struct TicketFilter {
    pub project_id: Option<u64>,
}
