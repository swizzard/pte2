#![recursion_limit = "512"]
pub mod from_toml;
pub mod to_html;
use crate::from_toml::read_exercises;
use crate::to_html::write_html;
use anyerror::AnyError;

pub fn write_html_from_config() -> Result<(), AnyError> {
    let exs = read_exercises()?;
    let h = write_html(exs);
    std::fs::write("public/index.html", h).map_err(|e| AnyError::new(&e))?;
    Ok(())
}
