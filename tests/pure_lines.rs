use pure_lines;

#[test]
fn test_pure_lines() {
    let doc = " \t\n    hello\n    world!";
    assert_eq!(pure_lines::pure(doc),String::from("hello\nworld!"));
}