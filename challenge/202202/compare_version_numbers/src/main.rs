fn compare_version(version1: String, version2: String) -> i32 {
    let to_str = |s: &str| -> i32 { s.bytes().fold(0, |acc, b| acc * 10 + (b - b'0') as i32) };

    let f = |s: &String| -> Vec<i32> {
        let mut ret: Vec<i32> = s.split('.').map(|ss| to_str(ss)).collect();

        for _ in ret.len()..3 {
            ret.push(0);
        }

        ret
    };

    let v1 = f(&version1);
    let v2 = f(&version2);

    for (n1, n2) in v1.into_iter().zip(v2) {
        if n1 < n2 {
            return -1;
        }

        if n1 > n2 {
            return 1;
        }
    }

    0
}

fn main() {
    let version1 = "1.01".to_string();
    let version2 = "1.0001".to_string();
    println!("ret={}", compare_version(version1, version2));
}

#[test]
fn test_compare_version() {
    {
        let version1 = "1.01".to_string();
        let version2 = "1.0001".to_string();
        assert_eq!(compare_version(version1, version2), 0);
    }
    {
        let version1 = "1.0".to_string();
        let version2 = "1.0.0".to_string();
        assert_eq!(compare_version(version1, version2), 0);
    }
    {
        let version1 = "2.0".to_string();
        let version2 = "1.0".to_string();
        assert_eq!(compare_version(version1, version2), 1);
    }
    {
        let version1 = "0.1".to_string();
        let version2 = "1.1".to_string();
        assert_eq!(compare_version(version1, version2), -1);
    }
}
