fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let len = (n + 1) as usize;
    let mut v = vec![0; len];
    let mut check = vec![false; len];
    for t in trust.iter() {
        v[t[1] as usize] += 1;
        check[t[0] as usize] = true
    }

    for i in 1..=n {
        if v[i as usize] == n - 1 && !check[i as usize] {
            return i;
        }
    }

    -1
}

fn main() {
    let trust = vec![vec![1, 3], vec![2, 3]];
    println!("ret={}", find_judge(3, trust));
}

#[test]
fn test_find_judge() {
    {
        let trust = vec![vec![1, 2]];
        assert_eq!(find_judge(2, trust), 2);
    }
    {
        let trust = vec![vec![1, 3], vec![2, 3]];
        assert_eq!(find_judge(3, trust), 3);
    }
    {
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        assert_eq!(find_judge(3, trust), -1);
    }
    {
        let trust = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
        assert_eq!(find_judge(4, trust), 3);
    }
}
