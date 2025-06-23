fn check_prime_frequency(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;

    let mut primes = [true; 101];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=100 {
        if !primes[i] {
            continue;
        }
        primes[i] = true;
        let mut v = i * 2;
        while v <= 100 {
            primes[v] = false;
            v += i;
        }
    }

    let mut h = HashMap::new();
    for num in nums {
        *h.entry(num).or_insert(0) += 1;
    }

    for &v in h.values() {
        if primes[v as usize] {
            return true;
        }
    }

    false
}

fn main() {
    let nums = vec![3,0,3,6,3,3];
    let ret = check_prime_frequency(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4, 5, 4];
        assert!(check_prime_frequency(nums));
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        assert!(!check_prime_frequency(nums));
    }
    {
        let nums = vec![2, 2, 2, 4, 4];
        assert!(check_prime_frequency(nums));
    }
    {
        let nums = vec![3, 0, 3, 6, 3, 3];
        assert!(!check_prime_frequency(nums));
    }
}
