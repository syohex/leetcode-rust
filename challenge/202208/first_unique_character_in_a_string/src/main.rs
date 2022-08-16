fn first_uniq_char(s: String) -> i32 {
    let indices = s
        .bytes()
        .enumerate()
        .fold(vec![vec![]; 26], |mut acc, (i, b)| {
            let index = (b - b'a') as usize;
            acc[index].push(i);
            acc
        });

    let mut min = s.len();
    for x in indices {
        if x.len() == 1 {
            min = std::cmp::min(min, x[0]);
        }
    }

    if min == s.len() {
        -1
    } else {
        min as i32
    }
}

fn main() {
    let s = "loveleetcode".to_string();
    let ret = first_uniq_char(s);
    println!("ret={ret}");
}

#[test]
fn test_first_uniq_char() {
    {
        let s = "leetcode".to_string();
        let ret = first_uniq_char(s);
        assert_eq!(ret, 0);
    }
    {
        let s = "loveleetcode".to_string();
        let ret = first_uniq_char(s);
        assert_eq!(ret, 2);
    }
    {
        let s = "aabb".to_string();
        let ret = first_uniq_char(s);
        assert_eq!(ret, -1);
    }
    {
        let s = "".to_string();
        let ret = first_uniq_char(s);
        assert_eq!(ret, -1);
    }
}
