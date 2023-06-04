use count_loc_action::fs::{get_env, get_input, parse_multivalued_env};
use count_loc_action::loc::get_loc;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{env, error};

fn run() -> Result<(), Box<dyn error::Error>> {
    let github_workspace = get_env("GITHUB_WORKSPACE")?;
    let output_file = get_env("GITHUB_OUTPUT")?;
    env::set_current_dir(Path::new(github_workspace.as_str()))?;
    let mut paths = vec![".".to_string()];
    let mut excluded = vec![];

    // Count Loc
    if let Some(paths_env) = get_input("paths") {
        paths = parse_multivalued_env(paths_env.as_str())?;
    }
    if let Some(excluded_env) = get_input("excluded") {
        excluded = parse_multivalued_env(excluded_env.as_str())?;
    }
    let outputs = get_loc(
        &paths,
        excluded
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    )?;
    let mut file = OpenOptions::new().append(true).open(&output_file)?;
    for output in outputs {
        writeln!(file, "{}_{}={}", output.language, "code", output.code)?;
        writeln!(file, "{}_{}={}", output.language, "blanks", output.blanks)?;
        writeln!(
            file,
            "{}_{}={}",
            output.language, "comments", output.comments
        )?;
    }
    file.flush()?;
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
