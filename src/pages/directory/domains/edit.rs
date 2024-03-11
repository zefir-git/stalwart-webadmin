use std::sync::Arc;

use leptos::*;
use leptos_router::use_navigate;

use crate::{
    components::{
        form::{
            button::Button, input::InputText, Form, FormButtonBar, FormElement, FormItem,
            FormSection,
        },
        messages::alert::{use_alerts, Alert},
        Color,
    },
    core::{
        http::HttpRequest,
        oauth::use_authorization,
        schema::{Builder, Schemas, Transformer, Type, Validator},
    },
};

#[component]
pub fn DomainCreate() -> impl IntoView {
    let auth = use_authorization();
    let alert = use_alerts();

    let (pending, set_pending) = create_signal(false);

    let data = expect_context::<Arc<Schemas>>()
        .build_form("domains")
        .into_signal();

    let save_changes = create_action(move |name: &String| {
        let auth = auth.get();
        let name = name.clone();

        async move {
            set_pending.set(true);
            let result = HttpRequest::post(format!("/api/domain/{name}"))
                .with_authorization(&auth)
                .send::<()>()
                .await
                .map(|_| ());
            set_pending.set(false);

            match result {
                Ok(_) => {
                    use_navigate()("/manage/directory/domains", Default::default());
                }
                Err(err) => {
                    alert.set(Alert::from(err));
                }
            }
        }
    });

    view! {
        <Form title="Create domain" subtitle="Create a new local domain name">

            <FormSection>
                <FormItem label="Domain name">
                    <InputText
                        placeholder="example.org"

                        element=FormElement::new("domain", data)
                    />
                </FormItem>

            </FormSection>

            <FormButtonBar>
                <Button
                    text="Cancel"
                    color=Color::Gray
                    on_click=move |_| {
                        use_navigate()("/manage/directory/domains", Default::default());
                    }
                />

                <Button
                    text="Create"
                    color=Color::Blue
                    on_click=Callback::new(move |_| {
                        data.update(|data| {
                            if data.validate_form() {
                                save_changes.dispatch(data.value("domain").unwrap());
                            }
                        });
                    })

                    disabled=pending
                />
            </FormButtonBar>

        </Form>
    }
}

impl Builder<Schemas, ()> {
    pub fn build_domains(self) -> Self {
        self.new_schema("domains")
            .new_field("domain")
            .typ(Type::Input)
            .input_check(
                [Transformer::RemoveSpaces, Transformer::Lowercase],
                [Validator::Required, Validator::IsDomain],
            )
            .build()
            .build()
    }
}
