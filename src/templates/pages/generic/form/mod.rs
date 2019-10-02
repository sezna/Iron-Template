use horrorshow::helper::doctype;
use horrorshow::prelude::*;

mod form_field;
pub mod templates;
pub use self::form_field::{FieldType, FormField};

use store::User;
use templates::components::{header, nav_bar};

// The "default value" property doesn't work
// for select dropdowns. There are some details about avoiding extra clones here    :
// https://docs.rs/horrorshow/0.7.0/horrorshow/#returning-templates

#[derive(Clone)]
pub struct Form {
    /// The header/title of the page
    title: String,
    /// The individual (name, type, helpText (optional)) pairings -- e.g. ("username", "text", "please enter your username"
    fields: Vec<FormField>,
    /// An optional path to custom CSS
    css_path: Option<String>,
    /// The path to the action/endpoint this form should hit.
    action: String,
    /// The text that is inside the submit button
    submit_button_text: String,
    /// Any errors that have been thrown in this form.
    errors: Vec<String>,
}
impl Form {
    pub fn new(
        title: &str,
        fields: Vec<FormField>,
        action: &str,
        submit_button_text: &str,
    ) -> Form {
        Form {
            title: String::from(title),
            fields,
            css_path: None,
            action: String::from(action),
            submit_button_text: String::from(submit_button_text),
            errors: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn new_custom_css(
        title: &str,
        fields: Vec<FormField>,
        action: &str,
        css_path: &str,
        submit_button_text: &str,
    ) -> Form {
        Form {
            title: String::from(title),
            fields,
            css_path: Some(String::from(css_path)),
            action: String::from(action),
            submit_button_text: String::from(submit_button_text),
            errors: Vec::new(),
        }
    }

    /// Add an error message to the form.
    pub fn add_error(&mut self, error: &str) {
        self.errors.push(String::from(error));
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn set_defaults(&mut self, defaults: Vec<Option<&str>>) {
        assert_eq!(
            defaults.len(),
            self.fields.len(),
            "Attempted to set defaults on a form but passed in the wrong number of defaults"
        );
        for i in 0..defaults.len() {
            self.fields[i].set_default_value(defaults[i]);
        }
    }

    /// A generic form component which takes in a list of [[FormProperty]]s (how do I pluralize correctly while maintaining
    /// the global link to [[FormProperty]]?)
    pub fn render(&self, user: Box<Option<User>>) -> String {
        let css_path = if let Some(path) = self.clone().css_path {
            path
        } else {
            String::from("/files/css/practice_sheet_form.css")
        };
        let markup = html! {
        : doctype::HTML;
            html {
                :header(String::from(self.clone().title));
                link(rel="stylesheet", href=css_path);
                body {
                    : nav_bar(user);
                    @ if self.clone().errors.len() > 0 {
                        div(class="form-error") {
                            ol {
                                @ for e in self.clone().errors.iter() {
                                    li { : e }
                                }
                            }
                        }
                    }
                    form(class="generic-form", action=self.action.clone(), method="post") {
                        @ for field in self.fields.clone() {
                            : field.render();
                        }
                        input(type="submit", value=self.submit_button_text.clone());

                    }
                }
            }
        };
        return markup.into_string().unwrap();
    }
}
