fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    let mut primes = vec![true; right as usize + 1];
    primes[0] = false;
    primes[1] = false;

    for i in 2..=right as usize {
        if !primes[i] {
            continue;
        }

        let mut j = 2;
        while i * j <= right as usize {
            primes[i * j] = false;
            j += 1;
        }
    }

    let mut ret = vec![-1; 2];
    let mut prev1 = -1;
    let mut min_diff = std::i32::MAX;
    for i in left..=right {
        if primes[i as usize] {
            let prev2 = prev1;
            prev1 = i as i32;

            if prev1 != -1 && prev2 != -1 && prev1 - prev2 < min_diff {
                min_diff = prev1 - prev2;
                ret[0] = prev2;
                ret[1] = prev1;
            }
        }
    }

    ret
}

fn main() {
    let ret = closest_primes(10, 19);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let expected = vec![11, 13];
        let ret = closest_primes(10, 19);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![-1, -1];
        let ret = closest_primes(4, 6);
        assert_eq!(ret, expected);
    }
}
