fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    let is_x_overlapped =
        (rec1[2] > rec2[0] && rec1[0] < rec2[2]) || (rec2[2] > rec1[0] && rec2[0] < rec1[2]);
    let is_y_overlapped =
        (rec1[3] > rec2[1] && rec1[1] < rec2[3]) || (rec2[3] > rec1[1] && rec2[1] < rec1[3]);
    is_x_overlapped && is_y_overlapped
}

fn main() {
    let ret = is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]));
    assert!(!is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]));
    assert!(!is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]));
}
