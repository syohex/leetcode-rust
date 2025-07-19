fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    use std::collections::HashSet;

    let s = folder.iter().fold(HashSet::new(), |mut acc, f| {
        acc.insert(f.as_str());
        acc
    });

    let mut ret = vec![];
    for f in &folder {
        let mut substr: &str = f;
        let mut is_subfolder = false;
        while !substr.is_empty() {
            if let Some(pos) = substr.rfind('/') {
                substr = &substr[0..pos];
                if s.contains(substr) {
                    is_subfolder = true;
                    break;
                }
            }
        }
        if !is_subfolder {
            ret.push(f.clone());
        }
    }

    ret
}

fn main() {
    let folder = vec![
        "/a".to_string(),
        "/a/b".to_string(),
        "/c/d".to_string(),
        "/c/d/e".to_string(),
        "/c/f".to_string(),
    ];
    let ret = remove_subfolders(folder);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let folder = vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/c/d".to_string(),
            "/c/d/e".to_string(),
            "/c/f".to_string(),
        ];
        let expected = vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()];
        let ret = remove_subfolders(folder);
        assert_eq!(ret, expected);
    }
    {
        let folder = vec!["/a".to_string(), "/a/b/c".to_string(), "/a/b/d".to_string()];
        let expected = vec!["/a".to_string()];
        let ret = remove_subfolders(folder);
        assert_eq!(ret, expected);
    }
    {
        let folder = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
        ];
        let expected = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
        ];
        let ret = remove_subfolders(folder);
        assert_eq!(ret, expected);
    }
}
