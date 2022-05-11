fn count_vowel_strings(n: i32) -> i32 {
    use std::collections::HashMap;

    fn f(n: i32, vowels: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
        if n == 1 {
            return vowels;
        }

        if vowels == 1 {
            return 1;
        }

        if let Some(v) = cache.get(&(n, vowels)) {
            return *v;
        }

        let ret1 = f(n, vowels - 1, cache);
        let ret2 = f(n - 1, vowels, cache);
        let ret = ret1 + ret2;

        cache.insert((n, vowels), ret);
        ret
    }

    let mut cache = HashMap::new();
    f(n, 5, &mut cache)
}

fn main() {
    let ret = count_vowel_strings(33);
    println!("ret={ret}");
}

#[test]
fn test_count_vowel_strings() {
    assert_eq!(count_vowel_strings(1), 5);
    assert_eq!(count_vowel_strings(2), 15);
    assert_eq!(count_vowel_strings(33), 66045);
}
