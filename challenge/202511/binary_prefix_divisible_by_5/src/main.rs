fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut ret = vec![];
    let mut sum = 0;
    for n in nums {
        sum = (sum * 2 + n) % 5;
        ret.push(sum == 0);
    }

    ret
}

fn main() {
    let ret = prefixes_div_by5(vec![0, 1, 1, 1, 0, 1, 0, 1]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(prefixes_div_by5(vec![0, 1, 1]), [true, false, false]);
    assert_eq!(prefixes_div_by5(vec![1, 1, 1]), [false, false, false]);
}
