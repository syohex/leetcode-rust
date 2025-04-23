fn count_largest_group(n: i32) -> i32 {
    use std::collections::HashMap;

    fn to_digit_sum(mut n: i32) -> i32 {
        let mut ret = 0;
        while n > 0 {
            ret += n % 10;
            n /= 10;
        }

        ret
    }

    let mut h = HashMap::new();
    for i in 1..=n {
        let sum = to_digit_sum(i);
        *h.entry(sum).or_insert(0) += 1;
    }

    let largest = h.values().max().unwrap();
    h.values().filter(|&v| v == largest).count() as i32
}

fn main() {
    let ret = count_largest_group(13);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_largest_group(13), 4);
    assert_eq!(count_largest_group(2), 2);
}
