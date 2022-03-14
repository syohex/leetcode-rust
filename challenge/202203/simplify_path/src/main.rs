fn simplify_path(path: String) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    let mut stack = vec![];

    for part in parts {
        if part == "" || part == "." {
            // do nothing
        } else if part == ".." {
            stack.pop();
        } else {
            stack.push(part);
        }
    }

    format!("/{}", stack.join("/"))
}
fn main() {
    let ret = simplify_path("/a//b////c/d//././/..".to_string());
    println!("ret={ret}");
}

#[test]
fn test_simplify_path() {
    assert_eq!(simplify_path("/home/".to_string()), "/home".to_string());
    assert_eq!(simplify_path("/../".to_string()), "/".to_string());
    assert_eq!(
        simplify_path("/home//foo/".to_string()),
        "/home/foo".to_string()
    );
    assert_eq!(
        simplify_path("/a//b////c/d//././/..".to_string()),
        "/a/b/c".to_string()
    );
}
