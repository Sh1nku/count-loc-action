use count_loc_action::loc::{get_loc, TokeiResult};
use std::env;
use tempdir::TempDir;

fn setup_project() -> TempDir {
    let tmp_dir = TempDir::new("").unwrap();

    let rust_file = tmp_dir.path().join("hello_world.rs");
    std::fs::write(
        &rust_file,
        "fn main() {\n    println!(\"Hello, world!\");\n}",
    )
    .unwrap();

    let python_file = tmp_dir.path().join("hello_world.py");
    std::fs::write(&python_file, "print(\"Hello, world!\")").unwrap();

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
            TokeiResult {
                language: "Python".to_string(),
                code: 1,
                blanks: 0,
                comments: 0,
            },
            TokeiResult {
                language: "Rust".to_string(),
                code: 3,
                blanks: 0,
                comments: 0,
            },
            TokeiResult {
                language: "Total".to_string(),
                code: 4,
                blanks: 0,
                comments: 0,
            },
        ]
    );
}
