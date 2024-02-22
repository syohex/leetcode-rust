fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let len = n + 1;
    let mut trustees = vec![0; len];
    let mut trusters = vec![false; len];

    for t in trust {
        trustees[t[1] as usize] += 1;
        trusters[t[0] as usize] = true;
    }

    for i in 1..=n {
        if trustees[i] == n - 1 && !trusters[i] {
            return i as i32;
        }
    }

    -1
}

fn main() {
    let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
    let ret = find_judge(3, trust);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let trust = vec![vec![1, 2]];
        let ret = find_judge(2, trust);
        assert_eq!(ret, 2);
    }
    {
        let trust = vec![vec![1, 3], vec![2, 3]];
        let ret = find_judge(3, trust);
        assert_eq!(ret, 3);
    }
    {
        let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
        let ret = find_judge(3, trust);
        assert_eq!(ret, -1);
    }
    {
        let trust = vec![vec![1, 2], vec![2, 3]];
        let ret = find_judge(3, trust);
        assert_eq!(ret, -1);
    }
    {
        let trust = vec![];
        let ret = find_judge(1, trust);
        assert_eq!(ret, 1);
    }
}
