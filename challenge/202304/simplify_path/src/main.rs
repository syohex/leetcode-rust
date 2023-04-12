fn simplify_path(path: String) -> String {
    let mut ret: Vec<&str> = vec![""];

    for p in path.split("/") {
        if p.is_empty() || p == "." {
            // do nothing
        } else if p == ".." {
            if ret.len() != 1 {
                ret.pop();
            }
        } else {
            ret.push(p);
        }
    }

    if ret.len() == 1 {
        "/".to_string()
    } else {
        ret.join("/")
    }
}

fn main() {
    let path = "/a//b////c/d//././/..".to_string();
    let ret = simplify_path(path);
    println!("ret={ret}");
}

#[test]
fn test_simplify_path() {
    {
        let path = "/home/".to_string();
        let ret = simplify_path(path);
        assert_eq!(ret, "/home");
    }
    {
        let path = "/../".to_string();
        let ret = simplify_path(path);
        assert_eq!(ret, "/");
    }
    {
        let path = "/home//foo".to_string();
        let ret = simplify_path(path);
        assert_eq!(ret, "/home/foo");
    }
    {
        let path = "/a//b////c/d//././/..".to_string();
        let ret = simplify_path(path);
        assert_eq!(ret, "/a/b/c");
    }
}
