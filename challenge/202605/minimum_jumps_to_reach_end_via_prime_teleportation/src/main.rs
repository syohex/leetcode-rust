fn min_jumps(nums: Vec<i32>) -> i32 {
    use std::collections::{VecDeque, HashMap};

    let max_num = *nums.iter().max().unwrap() as usize;
    let mut prime_factors = vec![vec![]; max_num + 1];
    for i in 2..=max_num {
        if prime_factors[i].is_empty() {
            for j in (i..=max_num).step_by(i) {
                prime_factors[j].push(i as i32)
            }
        }
    }

    let mut graph: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        if prime_factors[*n as usize].len() == 1 {
            graph.entry(*n).or_default().push(i);
        }
    }

    let len = nums.len();
    let mut q = VecDeque::new();
    q.push_back(len - 1);

    let mut ret = 0;
    let mut visited = vec![false; len];
    loop {
        let q_len = q.len();
        for _ in 0..q_len {
            let node = q.pop_front().unwrap();
            if node == 0 {
                return ret;
            }

            if node >= 1 && !visited[node - 1]{
                visited[node - 1] = true;
                q.push_back(node - 1);
            }
            if node + 1 < len && !visited[node + 1] {
                visited[node + 1] = true;
                q.push_back(node + 1);
            }

            for &factor in &prime_factors[nums[node] as usize] {
                if let Some(nexts) = graph.remove(&factor) {
                    for next in nexts {
                        if !visited[next] {
                            visited[next] = true;
                            q.push_back(next);
                        }
                    }
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
        let nums = vec![25, 5, 7, 3, 25];
        let ret = min_jumps(nums);
        assert_eq!(ret, 2);
    }
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
