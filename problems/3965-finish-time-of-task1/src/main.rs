fn finish_time(n: i32, edges: Vec<Vec<i32>>, base_time: Vec<i32>) -> i64 {
    fn f(node: usize, parent: usize, base_time: &[i32], graph: &Vec<Vec<usize>>) -> i64 {
        let mut latest = i64::MIN;
        let mut earliest = i64::MAX;
        let mut is_leaf = true;

        for &next in &graph[node] {
            if next != parent {
                is_leaf = false;
                let v = f(next, node, base_time, graph);
                latest = std::cmp::max(latest, v);
                earliest = std::cmp::min(earliest, v);
            }
        }

        if is_leaf {
            base_time[node] as i64
        } else {
            (latest + latest - earliest) + base_time[node] as i64
        }
    }

    let mut graph = vec![vec![]; n as usize];
    for e in edges {
        graph[e[0] as usize].push(e[1] as usize);
        graph[e[1] as usize].push(e[0] as usize);
    }

    f(0, usize::MAX, &base_time, &graph)
}

fn main() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2]];
    let base_time = vec![9, 5, 3];
    let ret = finish_time(n, edges, base_time);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2]];
        let base_time = vec![9, 5, 3];
        let ret = finish_time(n, edges, base_time);
        assert_eq!(ret, 17);
    }
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![0, 2]];
        let base_time = vec![4, 7, 6];
        let ret = finish_time(n, edges, base_time);
        assert_eq!(ret, 12);
    }
    {
        let n = 4;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3]];
        let base_time = vec![5, 8, 2, 1];
        let ret = finish_time(n, edges, base_time);
        assert_eq!(ret, 18);
    }
}
