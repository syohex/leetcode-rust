fn maximum_total_damage(power: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for p in power {
        *h.entry(p as i64).or_insert(0i64) += 1;
    }

    let mut v = vec![];
    for (power, count) in &h {
        v.push((*power, *count));
    }
    v.sort_unstable();

    let mut dp = vec![0; v.len()];
    let mut ret = i64::MIN;
    for i in 0..v.len() {
        let mut m = 0;
        let mut j = 0;
        while j < i && v[j].0 < v[i].0 - 2 {
            m = std::cmp::max(m, dp[j]);
            j += 1;
        }

        dp[i] = m + v[i].0 * v[i].1;
        ret = std::cmp::max(ret, dp[i]);
    }

    ret
}

fn main() {
    let ret = maximum_total_damage(vec![7, 1, 6, 6]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum_total_damage(vec![1, 1, 3, 4]), 6);
    assert_eq!(maximum_total_damage(vec![7, 1, 6, 6]), 13);
}
