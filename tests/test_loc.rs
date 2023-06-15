use count_loc_action::loc::{get_loc, OutputResult};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use tempdir::TempDir;

fn setup_project() -> TempDir {
    let tmp_dir = TempDir::new("").unwrap();

    let rust_file = tmp_dir.path().join("hello_world.rs");
    std::fs::write(
        &rust_file,
        "fn main() {\n    println!(\"Hello, world!\");\n}",
    )
    .unwrap();

    let mut python_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(tmp_dir.path().join("hello_world.py"))
        .unwrap();
    for _ in 0..1000 {
        writeln!(python_file, "print(\"Hello, world!\")").unwrap();
    }
    python_file.flush().unwrap();

    tmp_dir
}

#[test]
fn test_get_loc() {
    let dir = setup_project();
    env::set_current_dir(dir.path()).unwrap();

    let loc = get_loc(&["."], &[]).unwrap();
    assert_eq!(
        loc,
        vec![
            OutputResult {
                language: "Python".to_string(),
                code: 1000,
                code_abbreviated: "1.0K".to_string(),
                code_pretty: "1 000".to_string(),
                blanks: 0,
                blanks_abbreviated: "0".to_string(),
                blanks_pretty: "0".to_string(),
                comments: 0,
                comments_abbreviated: "0".to_string(),
                comments_pretty: "0".to_string(),
            },
            OutputResult {
                language: "Rust".to_string(),
                code: 3,
                code_abbreviated: "3".to_string(),
                code_pretty: "3".to_string(),
                blanks: 0,
                blanks_abbreviated: "0".to_string(),
                blanks_pretty: "0".to_string(),
                comments: 0,
                comments_abbreviated: "0".to_string(),
                comments_pretty: "0".to_string(),
            },
            OutputResult {
                language: "Total".to_string(),
                code: 1003,
                code_abbreviated: "1.0K".to_string(),
                code_pretty: "1 003".to_string(),
                blanks: 0,
                blanks_abbreviated: "0".to_string(),
                blanks_pretty: "0".to_string(),
                comments: 0,
                comments_abbreviated: "0".to_string(),
                comments_pretty: "0".to_string(),
            },
        ]
    );
}
