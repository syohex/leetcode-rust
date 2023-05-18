fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let reachables = edges
        .into_iter()
        .fold(vec![false; n as usize], |mut acc, edge| {
            acc[edge[1] as usize] = true;
            acc
        });

    (0..n).filter(|i| !reachables[*i as usize]).collect()
}

fn main() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
    let ret = find_smallest_set_of_vertices(6, edges);
    println!("ret={ret:?}");
}

#[test]
fn test_find_smallest_set_of_vertices() {
    {
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];
        let expected = vec![0, 3];
        let ret = find_smallest_set_of_vertices(6, edges);
        assert_eq!(ret, expected);
    }
    {
        let edges = vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]];
        let expected = vec![0, 2, 3];
        let ret = find_smallest_set_of_vertices(5, edges);
        assert_eq!(ret, expected);
    }
}
