fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let n = n as usize;
    let k = k as usize;
    let label: Vec<_> = labels.chars().collect();
    let mut graph = vec![vec![]; n];
    for e in edges {
        let (from, to, weight) = (e[0] as usize, e[1] as usize, e[2]);
        graph[from].push((to, weight as i64));
    }

    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0i64), 0, 1));

    let mut min_weights = vec![vec![i64::MAX; k + 1]; n];
    min_weights[0][1] = 0;

    while let Some((Reverse(w), node, count)) = pq.pop() {
        if w > min_weights[node][count] {
            continue;
        }
        if node == n - 1 {
            return w as i32;
        }

        for (next, weight) in &graph[node] {
            let new_count = if label[node] == label[*next] {
                count + 1
            } else {
                1
            };
            if new_count > k {
                continue;
            }

            let new_weight = w + weight;
            if new_weight < min_weights[*next][new_count] {
                min_weights[*next][new_count] = new_weight;
                pq.push((Reverse(new_weight), *next, new_count));
            }
        }
    }

    -1
}

fn main() {
    let n = 3;
    let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]];
    let labels = "aab".to_string();
    let k = 1;
    let ret = shortest_path(n, edges, labels, k);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 3;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]];
        let labels = "aab".to_string();
        let k = 1;
        let ret = shortest_path(n, edges, labels, k);
        assert_eq!(ret, 3);
    }
    {
        let n = 3;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]];
        let labels = "aab".to_string();
        let k = 2;
        let ret = shortest_path(n, edges, labels, k);
        assert_eq!(ret, 2);
    }
    {
        let n = 3;
        let edges = vec![vec![0, 1, 1], vec![1, 2, 1]];
        let labels = "aaa".to_string();
        let k = 2;
        let ret = shortest_path(n, edges, labels, k);
        assert_eq!(ret, -1);
    }
}
