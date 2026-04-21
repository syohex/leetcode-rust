fn minimum_hamming_distance(
    source: Vec<i32>,
    target: Vec<i32>,
    allowed_swaps: Vec<Vec<i32>>,
) -> i32 {
    use std::collections::HashMap;

    fn f(
        index: usize,
        source: &[i32],
        target: &[i32],
        graph: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        acc: &mut HashMap<i32, i32>,
    ) {
        visited[index] = true;
        *acc.entry(source[index]).or_insert(0) += 1;
        *acc.entry(target[index]).or_insert(0) -= 1;

        for &next in &graph[index] {
            if !visited[next] {
                f(next, source, target, graph, visited, acc);
            }
        }
    }

    let len = source.len();
    let mut graph = vec![vec![]; len];
    for s in allowed_swaps {
        graph[s[0] as usize].push(s[1] as usize);
        graph[s[1] as usize].push(s[0] as usize);
    }

    let mut ret = 0;
    let mut visited = vec![false; len];
    for i in 0..len {
        if !visited[i] {
            let mut acc = HashMap::new();
            f(i, &source, &target, &graph, &mut visited, &mut acc);
            ret += acc.values().filter(|&n| *n > 0).sum::<i32>();
        }
    }

    ret
}

fn main() {
    let source = vec![2, 3, 1];
    let target = vec![1, 2, 2];
    let allowed_swaps = vec![vec![0, 2], vec![1, 2]];
    let ret = minimum_hamming_distance(source, target, allowed_swaps);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let source = vec![2, 3, 1];
        let target = vec![1, 2, 2];
        let allowed_swaps = vec![vec![0, 2], vec![1, 2]];
        let ret = minimum_hamming_distance(source, target, allowed_swaps);
        assert_eq!(ret, 1);
    }
    {
        let source = vec![1, 2, 3, 4];
        let target = vec![1, 3, 2, 4];
        let allowed_swaps = vec![];
        let ret = minimum_hamming_distance(source, target, allowed_swaps);
        assert_eq!(ret, 2);
    }
    {
        let source = vec![1, 2, 3, 4];
        let target = vec![2, 1, 4, 5];
        let allowed_swaps = vec![vec![0, 1], vec![2, 3]];
        let ret = minimum_hamming_distance(source, target, allowed_swaps);
        assert_eq!(ret, 1);
    }
    {
        let source = vec![5, 1, 2, 4, 3];
        let target = vec![1, 5, 4, 2, 3];
        let allowed_swaps = vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]];
        let ret = minimum_hamming_distance(source, target, allowed_swaps);
        assert_eq!(ret, 0);
    }
}
