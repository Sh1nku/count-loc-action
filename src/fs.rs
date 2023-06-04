use csv::ReaderBuilder;
use std::env;
use std::env::VarError;
use std::error::Error;

pub fn get_input(name: &str) -> Option<String> {
    match env::var(format!("INPUT_{}", name.to_uppercase())) {
        Ok(val) => match val.is_empty() {
            true => None,
            false => Some(val),
        },
        Err(_) => None,
    }
}

pub fn get_env(var: &str) -> Result<String, VarError> {
    env::var(var)
}

pub fn parse_multivalued_env(string: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match string.is_empty() {
        true => Ok(vec![]),
        false => Ok(ReaderBuilder::new()
            .has_headers(false)
            .from_reader(string.as_bytes())
            .into_records()
            .next()
            .ok_or("No paths found")??
            .into_iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()),
    }
}
