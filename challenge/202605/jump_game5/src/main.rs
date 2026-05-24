fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
    fn f(pos: i32, arr: &[i32], d: i32, dp: &mut Vec<i32>) {
        if dp[pos as usize] != -1 {
            return;
        }

        dp[pos as usize] = 1;

        let mut i = pos - 1;
        while i >= 0 && pos - i <= d {
            if arr[pos as usize] <= arr[i as usize] {
                break;
            }

            f(i, arr, d, dp);
            dp[pos as usize] = std::cmp::max(dp[pos as usize], dp[i as usize] + 1);
            i -= 1;
        }

        i = pos + 1;
        while i < arr.len() as i32 && i - pos <= d {
            if arr[pos as usize] <= arr[i as usize] {
                break;
            }

            f(i, arr, d, dp);
            dp[pos as usize] = std::cmp::max(dp[pos as usize], dp[i as usize] + 1);
            i += 1;
        }
    }

    let mut dp = vec![-1; arr.len()];
    let mut ret = 0;
    for i in 0..arr.len() {
        f(i as i32, &arr, d, &mut dp);
        ret = std::cmp::max(ret, dp[i]);
    }

    ret
}

fn main() {
    let ret = max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2), 4);
    assert_eq!(max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
    assert_eq!(max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
}
