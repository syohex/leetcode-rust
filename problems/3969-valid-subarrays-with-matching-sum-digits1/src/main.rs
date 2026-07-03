fn count_valid_subarrays(nums: Vec<i32>, x: i32) -> i32 {
    let len = nums.len();
    let mut prefix = vec![0i64; len + 1];
    for (i, n) in nums.into_iter().enumerate() {
        prefix[i + 1] = prefix[i] + n as i64;
    }

    let x = x as i64;
    let mut ret = 0;
    for i in 0..=len {
        for j in (i + 1)..=len {
            let mut sum = prefix[j] - prefix[i];
            let last = sum % 10;
            if last != x {
                continue;
            }

            let mut first = last;
            while sum > 0 {
                first = sum % 10;
                sum /= 10;
            }

            if first == x {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let ret = count_valid_subarrays(vec![1, 100, 1], 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        count_valid_subarrays(
            vec![1000000000, 1, 1000000000, 1, 1000000000, 1, 1000000000],
            3
        ),
        2
    );
    assert_eq!(count_valid_subarrays(vec![1, 100, 1], 1), 4);
    assert_eq!(count_valid_subarrays(vec![1], 2), 0);
}
