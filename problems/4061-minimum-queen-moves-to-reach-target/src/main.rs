fn min_queen_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
    if source[0] == target[0] && source[1] == target[1] {
        0
    } else if source[0] == target[0]
        || source[1] == target[1]
        || (source[0] - target[0]).abs() == (source[1] - target[1]).abs()
    {
        1
    } else {
        2
    }
}

fn main() {
    let ret = min_queen_moves(vec![4, 3], vec![3, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_queen_moves(vec![8, 1], vec![1, 8]), 1);
    assert_eq!(min_queen_moves(vec![4, 2], vec![1, 3]), 2);
    assert_eq!(min_queen_moves(vec![1, 1], vec![1, 1]), 0);
}
