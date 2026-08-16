fn elevator_requests(_n: i32, requests: Vec<i32>) -> i32 {
    let mut floor = 0;
    let mut moves = 0;
    for r in requests {
        moves += (floor - r).abs();
        floor = r;
    }

    moves
}

fn main() {
    let ret = elevator_requests(5, vec![2, 1, 4, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(elevator_requests(5, vec![2, 1, 4, 3]), 7);
    assert_eq!(elevator_requests(3, vec![2, 0, 0]), 4);
}
