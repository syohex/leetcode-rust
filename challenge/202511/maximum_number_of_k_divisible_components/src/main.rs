fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    fn f(node: i32, prev: i32, graph: &[Vec<i32>], values: &[i32], k: i32, acc: &mut i32) -> i32 {
        let mut sum = values[node as usize];

        for &next in &graph[node as usize] {
            if next != prev {
                sum += f(next, node, graph, values, k, acc);
                sum %= k;
            }
        }

        if sum % k == 0 {
            *acc += 1;
        }

        sum
    }

    let n = n as usize;
    let mut graph = vec![vec![]; n];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    }

    let mut acc = 0;
    f(0, -1, &graph, &values, k, &mut acc);
    acc
}

fn main() {
    let n = 5;
    let edges = vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]];
    let values = vec![1, 8, 1, 4, 4];
    let k = 6;
    let ret = max_k_divisible_components(n, edges, values, k);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 5;
        let edges = vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]];
        let values = vec![1, 8, 1, 4, 4];
        let k = 6;
        let ret = max_k_divisible_components(n, edges, values, k);
        assert_eq!(ret, 2);
    }
    {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ];
        let values = vec![3, 0, 6, 1, 5, 2, 1];
        let k = 3;
        let ret = max_k_divisible_components(n, edges, values, k);
        assert_eq!(ret, 3);
    }
}
