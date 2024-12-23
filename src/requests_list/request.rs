use std::fmt::Display;

use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::ListItem,
};

pub struct Request {
    name: String,
    method: Option<Method>,
}

#[derive(Clone)]
pub enum Method {
    GET,
    POST,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
        }
    }
}

impl Request {
    pub fn new(name: &str, method: Option<Method>) -> Self {
        return Request {
            name: name.to_string(),
            method,
        };
    }
}

impl<'a> From<&Request> for ListItem<'_> {
    fn from(value: &Request) -> Self {
        let method_styled = match &value.method {
            Some(method) => style_method(method),
            None => "NONE".to_string().gray(),
        };

        let name = format!("  {}", value.name);
        let name_with_method = Line::default().spans(vec![method_styled, name.into()]);

        return ListItem::new(name_with_method);
    }
}

fn style_method(method: &Method) -> Span<'static> {
    let mut method_string = format!("{}", method);

    method_string.push_str(&" ".repeat(4 - method_string.len()));

    return match method {
        Method::GET => method_string.green(),
        Method::POST => method_string.blue(),
    };
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;
    use test_case::test_case;

    use super::*;

    #[test_case(Method::GET, "GET")]
    #[test_case(Method::POST, "POST")]
    fn method_string_returns_correct_string_representation(
        method: Method,
        expected_method_string: &str,
    ) {
        let method_string = method.to_string();

        assert_eq!(method_string, expected_method_string);
    }

    #[test_case(Method::GET, Color::Green)]
    #[test_case(Method::POST, Color::Blue)]
    fn method_correctly_colored(method: Method, expected_color: Color) {
        let styled_method = style_method(&method);

        assert_eq!(styled_method.style.fg.unwrap(), expected_color);
    }

    #[test_case(Method::GET, "GET ")]
    #[test_case(Method::POST, "POST")]
    fn method_correctly_padded(method: Method, expected_padded_method: &str) {
        let padded = style_method(&method);

        assert_eq!(padded.to_string(), expected_padded_method);
    }
}
