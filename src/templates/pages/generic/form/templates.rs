use crate::templates::pages::generic::form::{FieldType, Form, FormField};

pub fn log_in() -> Form {
    Form::new(
        "Log In",
        vec![
            FormField::new("Username", FieldType::Text, Some("Username"), None),
            FormField::new("Password", FieldType::Password, Some("Password"), None),
            FormField::new("action", FieldType::Hidden("login".to_string()), None, None),
        ],
        "/session",
        "Log In",
    )
}

pub fn register() -> Form {
    Form::new(
        "Register",
        vec![
            FormField::new("Username", FieldType::Text, Some("Username"), None),
            FormField::new("Password", FieldType::Password, Some("Password"), None),
            FormField::new(
                "pconfirm",
                FieldType::Password,
                Some("Confirm Password"),
                None,
            ),
            FormField::new("Email", FieldType::Text, Some("Email Address"), None),
            FormField::new(
                "charpref",
                FieldType::Select(vec![
                    "Simplified".to_string(),
                    "Traditional".to_string(),
                    "Both".to_string(),
                ]),
                Some("Simplified or traditional characters?"),
                None,
            ),
            FormField::new(
                "phopref",
                FieldType::Select(vec![
                    "Pīnyīn".to_string(),
                    "ㄅㄆㄇㄈ".to_string(),
                    "Both".to_string(),
                ]),
                Some("Pīnyīn, zhuyin/ㄅㄆㄇㄈ, or both phonetic systems?"),
                None,
            ),
            FormField::new(
                "action",
                FieldType::Hidden("register".to_string()),
                None,
                None,
            ),
        ],
        "/session",
        "Register",
    )
}
