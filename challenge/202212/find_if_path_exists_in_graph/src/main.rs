pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut h: HashMap<i32, Vec<i32>> = HashMap::new();
    for edge in edges {
        let src = edge[0];
        let dest = edge[1];

        if let Some(v) = h.get_mut(&src) {
            v.push(dest);
        } else {
            h.insert(src, vec![dest]);
        }

        if let Some(v) = h.get_mut(&dest) {
            v.push(src);
        } else {
            h.insert(dest, vec![src]);
        }
    }

    let mut visited: HashSet<i32> = HashSet::new();
    let mut q = vec![source];
    while !q.is_empty() {
        let next = q.pop().unwrap();
        if next == destination {
            return true;
        }

        visited.insert(next);

        if let Some(v) = h.get(&next) {
            for node in v {
                if !visited.contains(node) {
                    q.push(*node);
                }
            }
        }
    }

    false
}

fn main() {
    let n = 3;
    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
    let source = 0;
    let destination = 2;
    println!("ret={}", valid_path(n, edges, source, destination));
}

#[test]
fn test_valid_path() {
    {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        assert!(valid_path(n, edges, source, destination));
    }
    {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let source = 0;
        let destination = 5;
        assert!(!valid_path(n, edges, source, destination));
    }
}
