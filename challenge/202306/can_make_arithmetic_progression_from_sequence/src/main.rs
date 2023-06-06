fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    let mut arr = arr;
    arr.sort_unstable();
    let diffs: Vec<i32> = arr.windows(2).map(|v| v[1] - v[0]).collect();

    diffs.iter().skip(1).all(|diff| *diff == diffs[0])
}

fn main() {
    let arr = vec![3, 5, 1];
    let ret = can_make_arithmetic_progression(arr);
    println!("ret={ret}");
}

#[test]
fn test_can_make_arithmetic_progression() {
    {
        let arr = vec![3, 5, 1];
        let ret = can_make_arithmetic_progression(arr);
        assert!(ret);
    }
    {
        let arr = vec![1, 2, 4];
        let ret = can_make_arithmetic_progression(arr);
        assert!(!ret);
    }
}
