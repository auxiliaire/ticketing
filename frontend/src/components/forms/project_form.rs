use crate::components::bulma::field::Field;
use crate::components::html::checkbox::Checkbox;
use crate::components::html::date_input::DateInput;
use crate::components::html::text_input::TextInput;
use chrono::{DateTime, NaiveDate, Utc};
use implicit_clone::unsync::IString;
use serde_valid::Validate;
use shared::api::error::error_response::ErrorResponse;
use shared::dtos::project::Project as ProjectDto;
use shared::validation::is_empty::IsEmpty;
use shared::validation::validation_messages::{
    ErrorsWrapper, IValidationMessages, ValidationMessagesTrait,
};
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<(ProjectDto, Callback<ErrorResponse>)>,
}

pub enum ProjectMsg {
    UpdateSummary(AttrValue),
    UpdateDeadline(AttrValue),
    UpdateOwner(AttrValue),
    UpdateActive(bool),
    Submit(),
    UpdateErrors(ErrorResponse),
    Cancel(),
}

pub struct ProjectForm {
    project: ProjectDto,
    deadline: IString,
    owner: IString,
    on_submit: Callback<(ProjectDto, Callback<ErrorResponse>)>,
    common_error: IValidationMessages,
    summary_error: IValidationMessages,
    deadline_error: IValidationMessages,
    owner_error: IValidationMessages,
}
impl Component for ProjectForm {
    type Message = ProjectMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            project: ProjectDto::default(),
            deadline: IString::from(""),
            owner: IString::from(""),
            on_submit: ctx.props().onsubmit.to_owned(),
            common_error: None,
            summary_error: None,
            deadline_error: None,
            owner_error: None,
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectMsg::UpdateSummary(summary) => {
                self.project.summary = String::from(summary.as_str());
            }
            ProjectMsg::UpdateDeadline(deadline) => {
                self.deadline = deadline;
                log::debug!("Trying to parse '{}'", self.deadline.as_str());
                match NaiveDate::parse_from_str(self.deadline.as_str(), "%F") {
                    Ok(d) => {
                        log::debug!("Successfully parsed '{}'", self.deadline.as_str());
                        self.project.deadline =
                            Some(DateTime::from_local(d.and_hms_opt(0, 0, 0).unwrap(), Utc));
                    }
                    Err(e) => log::debug!("Parse failed with '{}'", e.to_string()),
                }
            }
            ProjectMsg::UpdateOwner(value) => {
                self.owner = value;
                if let Ok(v) = self.owner.as_str().parse::<u64>() {
                    self.project.user_id = v;
                }
            }
            ProjectMsg::UpdateActive(active) => {
                self.project.active = match active {
                    true => 1,
                    false => 0,
                };
            }
            ProjectMsg::Submit() => {
                let result = self.project.validate();
                match result {
                    Ok(_) => self.on_submit.emit((
                        self.project.clone(),
                        ctx.link().callback(ProjectMsg::UpdateErrors),
                    )),
                    Err(e) => self.update_errors(ErrorsWrapper(e)),
                }
            }
            ProjectMsg::UpdateErrors(error_response) => {
                log::debug!("Error response: {}", error_response);
                if let Some(errors) = error_response.details {
                    self.update_errors(errors);
                }
            }
            ProjectMsg::Cancel() => {
                let navigator = ctx.link().navigator().unwrap();
                navigator.back();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_cancel_pressed = |_: MouseEvent| ProjectMsg::Cancel();
        let on_submit_pressed = |_: MouseEvent| ProjectMsg::Submit();
        html! {
            <div class="card">
                <div class="card-content">
                    if let Some(common_error) = &self.common_error {
                        <p class="help is-danger">
                            <ul>
                            {
                                common_error.iter().map(|message| {
                                    html!{<li>{ message }</li>}
                                }).collect::<Html>()
                            }
                            </ul>
                        </p>
                    }
                    <Field label="Summary" help={&self.summary_error}>
                        <TextInput value={self.project.summary.clone()} on_change={ctx.link().callback(ProjectMsg::UpdateSummary)} valid={self.summary_error.is_empty()} />
                    </Field>
                    <Field label="Deadline" help={&self.deadline_error}>
                        <div class="field">
                            <div class="control">
                                <DateInput value={self.deadline.clone()} on_change={ctx.link().callback(ProjectMsg::UpdateDeadline)} valid={self.deadline_error.is_empty()} />
                            </div>
                        </div>
                    </Field>
                    <Field label="Owner" help={&self.owner_error}>
                        <TextInput value={self.owner.clone()} on_change={ctx.link().callback(ProjectMsg::UpdateOwner)} valid={self.owner_error.is_empty()} />
                    </Field>
                    <div class="field">
                        <p class="control">
                            <label class="checkbox">
                                <Checkbox checked={self.project.active == 1} on_change={ctx.link().callback(ProjectMsg::UpdateActive)} />
                                <b>{ " Active" }</b>
                            </label>
                        </p>
                    </div>
                </div>
                <footer class="card-footer">
                    <div class="card-footer-item">
                        <div class="field is-grouped">
                            <div class="control">
                                <button class="button is-link" onmouseup={ctx.link().callback(on_submit_pressed)}>{ "Submit" }</button>
                            </div>
                            <div class="control">
                                <button class="button is-link is-light" onmouseup={ctx.link().callback(on_cancel_pressed)}>{ "Cancel" }</button>
                            </div>
                        </div>
                    </div>
                </footer>
            </div>
        }
    }
}

impl ProjectForm {
    fn update_errors<E>(&mut self, errors: E)
    where
        E: ValidationMessagesTrait,
    {
        self.common_error = errors.get_common_messages();
        self.summary_error = errors.get_property_messages("summary");
        self.deadline_error = errors.get_property_messages("ts_seconds_option");
        self.owner_error = errors.get_property_messages("user_id");
    }
}
