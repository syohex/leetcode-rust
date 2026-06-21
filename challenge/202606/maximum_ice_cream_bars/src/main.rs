fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let max = *costs.iter().max().unwrap() as usize;
    let mut count = vec![0; max + 1];
    for &c in &costs {
        count[c as usize] += 1;
    }

    for i in 1..=max {
        count[i] += count[i - 1];
    }

    let mut sorted = vec![0; costs.len()];
    for c in costs.into_iter().rev() {
        sorted[count[c as usize] - 1] = c;
        count[c as usize] -= 1;
    }

    let mut ret = 0;
    let mut coins = coins;
    for c in sorted {
        if c > coins {
            break;
        }

        coins -= c;
        ret += 1;
    }

    ret
}

fn main() {
    let ret = max_ice_cream(vec![1, 3, 2, 4, 1], 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_ice_cream(vec![6, 2, 8, 8, 5, 6, 6, 2, 2, 2], 77), 10);
    assert_eq!(max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20), 6);
    assert_eq!(max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
    assert_eq!(max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5), 0);
}
