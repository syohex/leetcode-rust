fn min_merged_integer(nums: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut tmp = vec![];
    while i < nums.len() {
        let n = format!("{}{}", nums[i], nums[i + 1]);
        let m: i32 = n.parse().unwrap();
        tmp.push(m);
        i += 2;
    }

    tmp.into_iter().min().unwrap()
}

fn main() {
    let ret = min_merged_integer(vec![23, 55, 42, 21]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_merged_integer(vec![23, 55, 42, 21]), 2355);
    assert_eq!(min_merged_integer(vec![34, 7, 11, 11]), 347);
    assert_eq!(min_merged_integer(vec![9, 20]), 920);
    assert_eq!(min_merged_integer(vec![406, 911]), 406911);
}
