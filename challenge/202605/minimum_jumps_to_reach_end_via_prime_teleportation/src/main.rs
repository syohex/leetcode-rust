fn min_jumps(nums: Vec<i32>) -> i32 {
    use std::collections::VecDeque;

    fn is_prime(n: i32) -> bool {
        match n {
            _ if n <= 1 => false,
            2 => true,
            n => {
                let mut i = 3;
                while i * i <= n {
                    if n % i == 0 {
                        return false;
                    }
                    i += 2;
                }

                n % 2 != 0
            }
        }
    }

    let len = nums.len();
    let mut graph = vec![vec![]; len];
    for (i, n) in nums.iter().enumerate() {
        if i >= 1 {
            graph[i].push(i - 1);
        }
        if i + 1 < len {
            graph[i].push(i + 1);
        }
        if is_prime(*n) {
            for (j, m) in nums.iter().enumerate() {
                if i != j && *m % *n == 0 {
                    graph[i].push(j);
                }
            }
        }
    }

    let mut q = VecDeque::new();
    q.push_back(0);

    let mut ret = 0;
    let mut visited = vec![false; len];
    loop {
        let q_len = q.len();
        for _ in 0..q_len {
            let next = q.pop_front().unwrap();
            if next == len - 1 {
                return ret;
            }

            visited[next] = true;

            for n in &graph[next] {
                if !visited[*n] {
                    q.push_back(*n);
                }
            }
        }

        ret += 1;
    }
}

fn main() {
    let nums = vec![4, 6, 5, 8];
    let ret = min_jumps(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 4, 6];
        let ret = min_jumps(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![2, 3, 4, 7, 9];
        let ret = min_jumps(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![4, 6, 5, 8];
        let ret = min_jumps(nums);
        assert_eq!(ret, 3);
    }
}
