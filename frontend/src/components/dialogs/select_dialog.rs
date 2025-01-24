use super::super::option_data::OptionData;
use crate::app_state::{AppState, AppStateContext};
use crate::helpers::event_helper::get_values_from_select_change;
use implicit_clone::{sync::IArray, ImplicitClone};
use std::sync::Arc;
use yew::{
    html, html::onchange::Event, Callback, Component, Context, ContextHandle, Html, Properties,
};

pub enum SelectDialogMsg<V> {
    FetchedOptions(Vec<V>),
    ContextChanged(AppStateContext),
    ToggleOption(Event),
    SelectOptions(),
    CloseDialog,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SelectDialogProps<K, V>
where
    K: 'static + std::cmp::PartialEq + ImplicitClone,
    V: std::cmp::PartialEq + std::fmt::Display + std::clone::Clone,
{
    pub optionsapi: Callback<Callback<Vec<V>>>,
    pub onselect: Callback<IArray<K>>,
}

pub struct SelectDialog<K, V>
where
    K: 'static + ImplicitClone,
{
    options: Vec<V>,
    selected_keys: IArray<K>,
    app_state: AppStateContext,
    _listener: ContextHandle<AppStateContext>,
}

impl<K, V> Component for SelectDialog<K, V>
where
    K: 'static
        + std::cmp::PartialEq
        + std::fmt::Display
        + std::str::FromStr
        + std::clone::Clone
        + ImplicitClone,
    V: 'static + std::cmp::PartialEq + std::fmt::Display + std::clone::Clone + OptionData,
{
    type Message = SelectDialogMsg<V>;
    type Properties = SelectDialogProps<K, V>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.props()
            .optionsapi
            .emit(ctx.link().callback(SelectDialogMsg::FetchedOptions));
        let (app_state, _listener) = ctx
            .link()
            .context::<AppStateContext>(ctx.link().callback(SelectDialogMsg::ContextChanged))
            .expect("context to be set");
        Self {
            options: vec![],
            selected_keys: IArray::Rc(Arc::from(vec![])),
            app_state,
            _listener,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        ctx.props()
            .optionsapi
            .emit(ctx.link().callback(SelectDialogMsg::FetchedOptions));
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SelectDialogMsg::FetchedOptions(options) => {
                self.options = options;
            }
            SelectDialogMsg::ContextChanged(state) => {
                self.app_state = state;
            }
            SelectDialogMsg::ToggleOption(e) => {
                self.selected_keys = get_values_from_select_change::<K>(e);
            }
            SelectDialogMsg::SelectOptions() => {
                ctx.props().onselect.emit(self.selected_keys.clone());
            }
            SelectDialogMsg::CloseDialog => AppState::close_dialog(&self.app_state),
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let select_options = self.options.iter().map(move |v| {
            html! { <option value={v.get_key()}>{v.get_label()}</option> }
        });

        let on_close_click = ctx.link().callback(|_| SelectDialogMsg::CloseDialog);
        let on_select_change = ctx.link().callback(SelectDialogMsg::ToggleOption);
        let on_select = |_| SelectDialogMsg::SelectOptions();

        html! {
            <>
                <header class="modal-card-head">
                    <p class="modal-card-title">{ "Select tickets to assign" }</p>
                    <button class="delete" aria-label="close" onclick={on_close_click.clone()}></button>
                </header>
                <section class="modal-card-body">
                    <div class="select is-multiple is-fullwidth">
                        <select multiple={true} size="8" onchange={move |e| on_select_change.emit(e)}>
                            { for select_options }
                        </select>
                    </div>
                </section>
                <footer class="modal-card-foot">
                    <button class="button is-link" onclick={ctx.link().callback(on_select)}>{ "Save changes" }</button>
                    <button class="button is-link is-light" onclick={on_close_click}>{ "Cancel" }</button>
                </footer>
            </>
        }
    }
}
