use std::cmp::max;
use std::collections::HashMap;

fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for mut i in low_limit..=high_limit {
        let mut sum = 0;
        while i > 0 {
            sum += i % 10;
            i /= 10;
        }

        *h.entry(sum).or_insert(0) += 1;
    }

    let mut ret = 0;
    for (_, &v) in h.iter() {
        ret = max(ret, v);
    }

    ret
}

fn main() {
    println!("count_balls(1, 10)={}", count_balls(1, 10));
}

#[test]
fn test_count_balls() {
    assert_eq!(count_balls(1, 10), 2);
    assert_eq!(count_balls(5, 15), 2);
    assert_eq!(count_balls(19, 28), 2);
}
