fn simplify_path(path: String) -> String {
    let v: Vec<&str> = path
        .split('/')
        .filter(|&s| !s.is_empty() && s != ".")
        .collect();

    let mut stack: Vec<&str> = vec![];
    for s in v {
        match s {
            ".." => {
                stack.pop();
            }
            p => {
                stack.push(p);
            }
        }
    }

    String::from("/") + &stack.join("/")
}

fn main() {
    println!(
        "simplify_path('/home/')={}",
        simplify_path("/home/".to_string())
    );
}

#[test]
fn test_simplify_path() {
    assert_eq!(simplify_path("/home/".to_string()), "/home");
    assert_eq!(simplify_path("/../".to_string()), "/");
    assert_eq!(simplify_path("/home//foo".to_string()), "/home/foo");
    assert_eq!(simplify_path("/a/./b/../../c/".to_string()), "/c");
}
