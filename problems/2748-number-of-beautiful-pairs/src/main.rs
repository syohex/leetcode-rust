fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;

        loop {
            let tmp = a % b;
            if tmp == 0 {
                return b;
            }

            a = b;
            b = tmp;
        }
    }

    let pairs: Vec<(i32, i32)> = nums
        .into_iter()
        .map(|n| n.to_string().bytes().collect::<Vec<u8>>())
        .map(|bs| ((bs[0] - b'0') as i32, (*bs.last().unwrap() - b'0') as i32))
        .collect();

    let mut cache = HashSet::new();
    let len = pairs.len();
    let mut ret = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            if cache.contains(&(pairs[i].0, pairs[j].1)) || gcd(pairs[i].0, pairs[j].1) == 1 {
                cache.insert((pairs[i].0, pairs[j].1));
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![2, 5, 1, 4];
    let ret = count_beautiful_pairs(nums);
    println!("ret={ret}");
}

#[test]
fn test_count_beautiful_pairs() {
    {
        let nums = vec![2, 5, 1, 4];
        let ret = count_beautiful_pairs(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![11, 21, 12];
        let ret = count_beautiful_pairs(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![31, 25, 72, 79, 74];
        let ret = count_beautiful_pairs(nums);
        assert_eq!(ret, 7);
    }
}
