use yew::prelude::*;

use crate::api::user::UserApi;
use shared::dtos::user::User as UserDto;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

pub enum Msg {
    FetchedUser(UserDto),
}

pub struct User {
    user: UserDto,
}
impl Component for User {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        UserApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedUser));
        Self {
            user: UserDto::default(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        UserApi::fetch(ctx.props().id, ctx.link().callback(Msg::FetchedUser));
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchedUser(user) => {
                self.user = user;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { user } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &user.name }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-info">
                                <div class="content">
                                    <p class="title">{ "Role" }</p>
                                    <div class="content">
                                        { &user.role }
                                    </div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
