use count_loc_action::fs::parse_multivalued_env;

#[test]
fn matching_works_normally() {
    assert_eq!(parse_multivalued_env("a,b,c").unwrap(), vec!["a", "b", "c"]);
}

#[test]
fn matching_works_with_empty_string() {
    assert_eq!(parse_multivalued_env("").unwrap(), Vec::<String>::new());
}

#[test]
fn matching_works_with_spaces() {
    assert_eq!(
        parse_multivalued_env("\"a a\",\"b \"b").unwrap(),
        vec!["a a", "b b"]
    );
}

#[test]
fn matching_works_with_backslashes() {
    assert_eq!(parse_multivalued_env("\"a\\\"").unwrap(), vec!["a\\"]);
}
