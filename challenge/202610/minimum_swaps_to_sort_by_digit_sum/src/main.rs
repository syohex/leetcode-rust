fn min_swaps(nums: Vec<i32>) -> i32 {
    fn digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        sum
    }

    let mut v: Vec<_> = nums
        .into_iter()
        .enumerate()
        .map(|(i, n)| (digit_sum(n), n, i))
        .collect();

    v.sort_unstable();

    let len = v.len();
    let mut positions = vec![0; len];
    for i in 0..len {
        positions[v[i].2] = i;
    }

    let mut checked = vec![false; len];
    let mut moves = 0;
    for i in 0..len {
        if !checked[i] {
            moves += 1;
            let mut j = i;
            while !checked[j] {
                checked[j] = true;
                j = positions[j];
            }
        }
    }

    (len - moves) as i32
}

fn main() {
    let ret = min_swaps(vec![18, 43, 34, 16]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_swaps(vec![268835996, 65052660, 415128775]), 2);
    assert_eq!(min_swaps(vec![37, 100]), 1);
    assert_eq!(min_swaps(vec![22, 14, 33, 7]), 0);
    assert_eq!(min_swaps(vec![18, 43, 34, 16]), 2);
}
