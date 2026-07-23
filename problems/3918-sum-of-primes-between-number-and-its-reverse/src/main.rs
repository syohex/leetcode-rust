fn sum_of_primes_in_range(n: i32) -> i32 {
    let reverse = n
        .to_string()
        .chars()
        .rev()
        .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap() as i32);

    let start = std::cmp::min(n, reverse) as usize;
    let end = std::cmp::max(n, reverse) as usize;

    let mut primes = vec![true; end + 1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=end {
        if !primes[i] {
            continue;
        }

        let mut j = i * 2;
        while j <= end {
            primes[j] = false;
            j += i;
        }
    }

    let mut ret = 0;
    for i in start..=end {
        if primes[i] {
            ret += i;
        }
    }

    ret as i32
}

fn main() {
    let ret = sum_of_primes_in_range(13);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sum_of_primes_in_range(13), 132);
    assert_eq!(sum_of_primes_in_range(10), 17);
    assert_eq!(sum_of_primes_in_range(8), 0);
}
