fn prime_sub_operation(nums: Vec<i32>) -> bool {
    fn is_prime(n: usize) -> bool {
        let limit = (n as f64).sqrt() as usize;
        (2..=limit).all(|m| n % m != 0)
    }

    let max_val = *nums.iter().max().unwrap() as usize;
    let mut nearest_primes = vec![0; max_val + 1];

    for i in 2..=max_val {
        if is_prime(i) {
            nearest_primes[i] = i as i32;
        } else {
            nearest_primes[i] = nearest_primes[i - 1];
        }
    }

    let mut prev = 0;
    for num in nums {
        let diff = num - prev;
        if diff <= 0 {
            return false;
        }

        prev = num - nearest_primes[diff as usize - 1];
    }

    true
}

fn main() {
    let nums = vec![5, 8, 3];
    let ret = prime_sub_operation(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 9, 6, 10];
        assert!(prime_sub_operation(nums));
    }

    {
        let nums = vec![6, 8, 11, 12];
        assert!(prime_sub_operation(nums));
    }
    {
        let nums = vec![5, 8, 3];
        assert!(!prime_sub_operation(nums));
    }
}
