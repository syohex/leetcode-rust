fn min_operations(boxes: String) -> Vec<i32> {
    let cs: Vec<_> = boxes.chars().collect();

    let mut left_balls = 0;
    let mut right_balls = 0;
    let mut left_moves = 0;
    let mut right_moves = 0;

    let mut ret = vec![0; cs.len()];
    for i in 0..cs.len() {
        ret[i] += left_moves;
        left_balls += if cs[i] == '1' { 1 } else { 0 };
        left_moves += left_balls;

        ret[cs.len() - 1 - i] += right_moves;
        right_balls += if cs[cs.len() - 1 - i] == '1' { 1 } else { 0 };
        right_moves += right_balls;
    }

    ret
}

fn main() {
    let boxes = "001011".to_string();
    let ret = min_operations(boxes);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let boxes = "110".to_string();
        let expected = vec![1, 1, 3];
        let ret = min_operations(boxes);
        assert_eq!(ret, expected);
    }
    {
        let boxes = "001011".to_string();
        let expected = vec![11, 8, 5, 4, 3, 4];
        let ret = min_operations(boxes);
        assert_eq!(ret, expected);
    }
}
