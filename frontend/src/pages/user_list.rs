use crate::{components::user_card::UserCard, services::user_service::UserService};
use shared::dtos::user::User as UserDto;
use yew::prelude::*;

pub enum Msg {
    FetchedUsers(Vec<UserDto>),
}

pub struct UserList {
    list: Vec<UserDto>,
}
impl Component for UserList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        UserService::fetch_all(None, ctx.link().callback(Msg::FetchedUsers));
        Self { list: Vec::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedUsers(users) => {
                self.list = users;
            }
        }
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let users = self.list.iter().map(|user| {
            html! {
                <div class="tile is-parent">
                    <div class="tile is-child">
                        <UserCard name={user.name.clone()} id={user.id} />
                    </div>
                </div>
            }
        });

        html! {
            <div class="container">
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">{ "User list" }</h1>
                            <h2 class="subtitle">
                                { "Here you can see all users of the application" }
                            </h2>
                        </div>
                    </div>
                </section>
                <p class="section py-0">
                    { "This is the list of all the registered users of the application retrieved from the API in the background." }
                </p>
                <div class="section">
                    <div class="tile is-ancestor">
                        { for users }
                    </div>
                </div>
            </div>
        }
    }
}
