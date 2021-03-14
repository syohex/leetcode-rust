fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let v: Vec<i32> = edges[0].to_owned();
    *edges[1]
        .iter()
        .find(|&n| v.iter().find(|&m| *n == *m).is_some())
        .unwrap()
}

fn main() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
    println!("ret={}", find_center(edges));
}

#[test]
fn test_find_center() {
    {
        let edges = vec![vec![1, 2], vec![2, 3], vec![4, 2]];
        assert_eq!(find_center(edges), 2);
    }
    {
        let edges = vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]];
        assert_eq!(find_center(edges), 1);
    }
}
