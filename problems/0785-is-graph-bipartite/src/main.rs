fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let mut colors: Vec<i32> = vec![-1; graph.len()];
    for i in 0..graph.len() {
        if colors[i] != -1 {
            continue;
        }

        colors[i] = 1;
        let mut s: Vec<i32> = vec![];
        s.push(i as i32);

        while !s.is_empty() {
            let node = s.pop().unwrap();
            let index = node as usize;

            for &n in graph[index].iter() {
                let idx = n as usize;
                if colors[idx] == -1 {
                    colors[idx] = if colors[index] == 0 { 1 } else { 0 };
                    s.push(n);
                } else if colors[idx] == colors[index] {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
    println!("answer={}", is_bipartite(graph));
}

#[test]
fn test_is_bipartite() {
    {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert!(is_bipartite(graph));
    }
    {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert!(!is_bipartite(graph));
    }
}
