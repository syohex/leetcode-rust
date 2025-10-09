fn get_least_frequent_digit(n: i32) -> i32 {
    let mut n = n;
    let mut freq = [i32::MAX; 10];
    while n > 0 {
        let m = (n % 10) as usize;
        if freq[m] == i32::MAX {
            freq[m] = 1;
        } else {
            freq[m] += 1;
        }
        n /= 10;
    }

    let mut ret = 0;
    let mut min_freq = i32::MAX;
    for (i, n) in freq.into_iter().enumerate() {
        if n < min_freq {
            ret = i as i32;
            min_freq = n;
        }
    }

    ret
}

fn main() {
    let ret = get_least_frequent_digit(723344511);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(get_least_frequent_digit(1553322), 1);
    assert_eq!(get_least_frequent_digit(723344511), 2);
}
