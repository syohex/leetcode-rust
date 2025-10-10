fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let len = energy.len();
    let mut dp = vec![0; len];
    let k = k as usize;

    let mut ret = i32::MIN;
    for i in 1..=k {
        dp[len - i] = energy[len - i];
        ret = std::cmp::max(ret, dp[len - i]);
    }

    for i in (0..(len - k)).rev() {
        dp[i] = dp[i + k] + energy[i];
        ret = std::cmp::max(ret, dp[i]);
    }

    ret
}

fn main() {
    let ret = maximum_energy(vec![5, 2, -10, -5, 1], 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum_energy(vec![5, 2, -10, -5, 1], 3), 3);
    assert_eq!(maximum_energy(vec![-2, -3, -1], 2), -1);
}
