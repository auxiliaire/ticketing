use implicit_clone::unsync::IString;
use uuid::Uuid;
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
    User { id: Uuid },
    #[at("/users")]
    Users,
    #[at("/tickets/new")]
    TicketNew,
    #[at("/tickets/:id")]
    Ticket { id: u64 },
    #[at("/login")]
    Login,
    #[at("/verify/:token")]
    Verify { token: IString },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub struct RouteSelector {}

impl RouteSelector {
    pub fn is_protected(route: Route) -> bool {
        matches!(
            route,
            Route::ProjectBoard { id: _ }
                | Route::ProjectNew
                | Route::Project { id: _ }
                | Route::Projects
                | Route::User { id: _ }
                | Route::Users
                | Route::TicketNew
                | Route::Ticket { id: _ }
        )
    }

    pub fn is_public(route: Route) -> bool {
        !Self::is_protected(route)
    }
}
