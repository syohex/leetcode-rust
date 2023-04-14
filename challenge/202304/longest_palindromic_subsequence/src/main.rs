fn longest_palindrome_subseq(s: String) -> i32 {
    fn f(left: usize, right: usize, cs: &Vec<char>, cache: &mut Vec<Vec<i32>>) -> i32 {
        if left > right {
            return 0;
        }
        if left == right {
            return 1;
        }

        if cache[left][right] != -1 {
            return cache[left][right];
        }

        let ret = if cs[left] == cs[right] {
            f(left + 1, right - 1, cs, cache) + 2
        } else {
            let ret1 = f(left + 1, right, cs, cache);
            let ret2 = f(left, right - 1, cs, cache);
            std::cmp::max(ret1, ret2)
        };

        cache[left][right] = ret;
        ret
    }

    let cs: Vec<char> = s.chars().collect();
    let mut cache = vec![vec![-1; cs.len()]; cs.len()];
    f(0, cs.len() - 1, &cs, &mut cache)
}
fn main() {
    let s = "bbbab".to_string();
    let ret = longest_palindrome_subseq(s);
    println!("ret={ret}");
}

#[test]
fn test_longest_palindrome_subseq() {
    {
        let s = "bbbab".to_string();
        let ret = longest_palindrome_subseq(s);
        assert_eq!(ret, 4);
    }
    {
        let s = "cbbd".to_string();
        let ret = longest_palindrome_subseq(s);
        assert_eq!(ret, 2);
    }
}
