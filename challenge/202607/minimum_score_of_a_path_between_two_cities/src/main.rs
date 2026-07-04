fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut graph = vec![vec![]; (n + 1) as usize];
    for r in roads {
        graph[r[0] as usize].push((r[1] as usize, r[2]));
        graph[r[1] as usize].push((r[0] as usize, r[2]));
    }

    let mut ret = i32::MAX;
    let mut checked = vec![false; n as usize];
    let mut q = vec![1];
    q.push(1);
    checked[0] = true;

    while !q.is_empty() {
        let node = q.pop().unwrap();

        for (next, score) in &graph[node] {
            ret = std::cmp::min(ret, *score);
            if !checked[*next - 1] {
                checked[*next - 1] = true;
                q.push(*next);
            }
        }
    }

    ret
}

fn main() {
    let n = 4;
    let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
    let ret = min_score(n, roads);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 4;
        let roads = vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]];
        let ret = min_score(n, roads);
        assert_eq!(ret, 5);
    }
    {
        let n = 4;
        let roads = vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7]];
        let ret = min_score(n, roads);
        assert_eq!(ret, 2);
    }
}
