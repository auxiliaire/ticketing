use yew_router::Routable;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/projects/:id/board")]
    ProjectBoard { id: u64 },
    #[at("/projects/new")]
    ProjectNew,
    #[at("/projects/:id")]
    Project { id: u64 },
    #[at("/projects")]
    Projects,
    #[at("/register")]
    Registration,
    #[at("/users/:id")]
    User { id: u64 },
    #[at("/users")]
    Users,
    #[at("/tickets/new")]
    TicketNew,
    #[at("/tickets/:id")]
    Ticket { id: u64 },
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
