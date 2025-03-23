fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for edge in roads {
        let start = edge[0] as usize;
        let end = edge[1] as usize;
        let time = edge[2] as i64;

        graph[start].push((time, end));
        graph[end].push((time, start));
    }

    let modulo = 1_000_000_007;
    let mut times = vec![i64::MAX; n];
    let mut ret = vec![0i64; n];
    let mut q = BinaryHeap::new();

    times[0] = 0;
    ret[0] = 1;
    q.push(Reverse((0, 0)));
    while !q.is_empty() {
        let (time, node) = q.pop().unwrap().0;
        if time > times[node] {
            continue;
        }

        for &(weight, next) in &graph[node] {
            let w = time + weight;
            if w < times[next] {
                times[next] = w;
                ret[next] = ret[node];
                q.push(Reverse((w, next)));
            } else if w == times[next] {
                ret[next] = (ret[next] + ret[node]) % modulo;
            }
        }
    }

    ret[n - 1] as i32
}

fn main() {
    let n = 7;
    let roads = vec![
        vec![0, 6, 7],
        vec![0, 1, 2],
        vec![1, 2, 3],
        vec![1, 3, 3],
        vec![6, 3, 3],
        vec![3, 5, 1],
        vec![6, 5, 1],
        vec![2, 5, 1],
        vec![0, 4, 5],
        vec![4, 6, 2],
    ];
    let ret = count_paths(n, roads);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 7;
        let roads = vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ];
        let ret = count_paths(n, roads);
        assert_eq!(ret, 4);
    }
    {
        let n = 2;
        let roads = vec![vec![1, 0, 10]];
        let ret = count_paths(n, roads);
        assert_eq!(ret, 1);
    }
}
