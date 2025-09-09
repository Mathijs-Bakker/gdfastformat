#[cfg(test)]
mod tests {
    use gdfastformat::format_code;

    fn check(input: &str, expected: &str) {
        let formatted = format_code(input);
        assert_eq!(formatted.trim(), expected.trim());
    }

    #[test]
    fn test_simple_function() {
        check(
            r#"
func _ready():
    print("Hello World")
"#,
            r#"
func _ready():
    print("Hello World")
"#,
        );
    }

    #[test]
    fn test_if_statement() {
        check(
            r#"
func _process(delta):
    if delta > 1.0:
        print("Too slow!")
"#,
            r#"
func _process(delta):
    if delta > 1.0:
        print("Too slow!")
"#,
        );
    }

    #[test]
    fn test_nested_blocks() {
        check(
            r#"
class Player:
    func move():
        if is_moving:
            print("Moving")
"#,
            r#"
class Player:
    func move():
        if is_moving:
            print("Moving")
"#,
        );
    }

    #[test]
    fn test_comments_and_multiline_string() {
        check(
            r#"
# Top-level comment
func _ready():
    var s = """
Hello
World
"""
    print(s)  # inline comment
"#,
            r#"
# Top-level comment
func _ready():
    var s = """
Hello
World
"""
    print(s)  # inline comment
"#,
        );
    }
}
