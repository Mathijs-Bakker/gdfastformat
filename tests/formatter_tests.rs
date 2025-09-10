use gdfastformat::format_source;

#[test]
fn test_simple_function() {
    let source = r#"func _ready():
    print("Hello World")"#;

    let expected = r#"func _ready():
    print("Hello World")"#;

    let formatted = format_source(source);

    assert_eq!(formatted.trim_end(), expected.trim_end());
}

#[test]
fn test_if_statement() {
    let source = r#"func _process(delta):
    if delta > 1.0:
        print("Too slow!")"#;

    let expected = r#"func _process(delta):
    if delta > 1.0:
        print("Too slow!")"#;

    let formatted = format_source(source);

    assert_eq!(formatted.trim_end(), expected.trim_end());
}

#[test]
fn test_nested_blocks() {
    let source = r#"class Player:
    func move():
        if is_moving:
            print("Moving")"#;

    let expected = r#"class Player:
    func move():
        if is_moving:
            print("Moving")"#;

    let formatted = format_source(source);

    assert_eq!(formatted.trim_end(), expected.trim_end());
}

#[test]
fn test_comments_and_multiline_string() {
    let source = r#"# Top-level comment
func _ready():
    var s = """
Hello
World
"""
    print(s)  # inline comment"#;

    let expected = r#"# Top-level comment
func _ready():
    var s = """
Hello
World
"""
    print(s)  # inline comment"#;

    let formatted = format_source(source);

    assert_eq!(formatted.trim_end(), expected.trim_end());
}
