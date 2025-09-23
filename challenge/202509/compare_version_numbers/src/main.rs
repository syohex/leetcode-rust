fn compare_version(version1: String, version2: String) -> i32 {
    fn to_n(s: &str) -> i32 {
        s.bytes().fold(0, |acc, b| {
            acc * 10 + (b - b'0') as i32
        })
    }

    let v1 : Vec<_> = version1.split('.').collect();
    let v2 : Vec<_> = version2.split('.').collect();

    let len1 = v1.len();
    let len2 = v2.len();
    let mut i = 0;
    let mut j = 0;

    while i < len1 || j < len2 {
        let n1 = if i < len1 {
            to_n(v1[i])
        } else {
            0
        };
        let n2 = if j < len2 {
            to_n(v2[j])
        } else {
            0
        };

        if n1 < n2 {
            return -1;
        } else if n1 > n2 {
            return 1;
        }

        i += 1;
        j += 1;
    }

    0
}

fn main() {
    let ret = compare_version("3.14".to_string(), "42".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(compare_version("1.0.1".to_string(), "1".to_string()), 1);
    assert_eq!(compare_version("9.8".to_string(), "8.10".to_string()), 1);
    assert_eq!(compare_version("1.2".to_string(), "1.10".to_string()), -1);
    assert_eq!(compare_version("1.01".to_string(), "1.001".to_string()), 0);
    assert_eq!(compare_version("1.0".to_string(), "1.0.0.0".to_string()), 0);
}
