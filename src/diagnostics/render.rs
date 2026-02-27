use std::fmt;

#[allow(dead_code)]
pub fn render(err: &impl fmt::Display) -> String {
    err.to_string()
}
