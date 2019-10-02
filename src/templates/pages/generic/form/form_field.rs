use horrorshow::prelude::*;

#[derive(Clone)]
pub struct FormField {
    /// The name of the field when it gets submitted
    name: String,
    /// The type of the field. See [[FieldType]].
    field_type: FieldType,
    /// Optional help text. Appears in a `div` above the field.
    help_text: Option<String>,
    /// The optional default value of the field
    default_value: Option<String>,
    /// Whether or not the field is required.
    required: bool,
}

impl<'a> FormField {
    pub fn new(
        name: &str,
        field_type: FieldType,
        help_text: Option<&str>,
        default_value: Option<&str>,
    ) -> FormField {
        FormField {
            name: String::from(name),
            field_type,
            help_text: if let Some(text) = help_text {
                Some(String::from(text))
            } else {
                None
            },
            default_value: if let Some(text) = default_value {
                Some(String::from(text))
            } else {
                None
            },
            required: false,
        }
    }

    pub fn set_default_value(&mut self, value: Option<&str>) {
        self.default_value = if let Some(string) = value {
            Some(String::from(string))
        } else {
            None
        };
    }

    /// Renders form fields in the [[generic_form]].
    pub fn render(&'a self) -> Box<dyn Render + 'a> {
        let select_fields: Option<Vec<String>> = if let FieldType::Select(fields) = &self.field_type
        {
            Some(fields.to_vec())
        } else {
            None
        };
        let select_map_fields: Option<Vec<(String, String)>> =
            if let FieldType::SelectMap(fields) = &self.field_type {
                Some(fields.to_vec())
            } else {
                None
            };
        let hidden_value: Option<String> = if let FieldType::Hidden(val) = &self.field_type {
            Some(val.clone())
        } else {
            None
        };
        box_html! {
            @ if self.help_text.is_some() {
                div(class="form-info") {
                    : self.help_text.as_ref().unwrap()
                }
             }
            @ if self.field_type == FieldType::Text {
                input(type="text",
                      class="num-grey-selector",
                      name=self.name.to_lowercase(),
                      value=self.default_value.as_ref().unwrap_or(&String::new()),
                      required?=self.required)
            } else if self.field_type == FieldType::Textarea {
                   textarea(name=self.name.to_lowercase(),
                       class="char-input-textarea",
                       value=self.default_value.as_ref().unwrap_or(&String::new()),
                       required?=self.required)
            } else if self.field_type == FieldType::Select(select_fields.clone().unwrap_or(Vec::new())) {
                   select(class="font-selector",
                       name=self.name.to_lowercase(),
                       required?=self.required) {
                            @ for opt in select_fields.clone().unwrap().iter() {
                                option(value=opt.to_lowercase(),
                                       selected?=opt.to_lowercase() == self.default_value.clone().unwrap_or(String::new()).to_lowercase()
                                ) { : opt; }
                            }
                    }
            } else if self.field_type == FieldType::Hidden(hidden_value.clone().unwrap_or(String::from(""))) {
                   input(type="hidden", name=self.name.to_lowercase(), value=hidden_value.clone().unwrap())
            } else if self.field_type == FieldType::Password {
                   input(type="password", name=self.name.to_lowercase())
            } else if self.field_type == FieldType::SelectMap(select_map_fields.clone().unwrap_or(Vec::new())) {
                   select(class="font-selector",
                          name=self.name.to_lowercase(),
                          required?=self.required) {
                            @ for (opt_display, opt_value) in select_map_fields.clone().unwrap().iter() {
                                option(value=opt_value.clone().to_lowercase(),
                                       selected?=opt_value.to_lowercase() == self.default_value.clone().unwrap_or(String::new()).to_lowercase()
                                       ) { : opt_display }
                            }
                          }

            }
        }
    }
}
#[derive(Clone, PartialEq)]
pub enum FieldType {
    /// A typical text input type
    Text,
    /// A typical <textarea>
    Textarea,
    /// A select dropdown. Requires an array of values which will be used for both
    /// the "name" and the "value"
    Select(Vec<String>),
    /// A select dropdown which has different values for an option's display name and "value", i.e.
    /// a mapping
    SelectMap(Vec<(String, String)>),
    /// The hidden input type.
    Hidden(String),
    /// A password field, which will obscure text as it is typed.
    Password,
}
