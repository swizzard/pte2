use anyerror::AnyError;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Debug, Deserialize)]
pub struct Exercise {
    pub label: String,
    pub reps: u8,
}

#[derive(Debug, Deserialize)]
pub struct Exercises {
    pub exercises: Vec<Exercise>,
}

pub fn read_exercises() -> Result<Exercises, AnyError> {
    let s = read_to_string("exercises.toml").map_err(|e| AnyError::new(&e))?;
    let exercises: Exercises = toml::from_str(s.as_str()).map_err(|e| AnyError::new(&e))?;
    Ok(exercises)
}
