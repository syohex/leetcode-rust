fn transform_array(nums: Vec<i32>) -> Vec<i32> {
    let (odds, evens) = nums.into_iter().fold((0, 0), |(odds, evens), n| {
        if n % 2 == 0 {
            (odds, evens + 1)
        } else {
            (odds + 1, evens)
        }
    });

    let mut ret = vec![];
    ret.extend(std::iter::repeat_n(0, evens));
    ret.extend(std::iter::repeat_n(1, odds));
    ret
}

fn main() {
    let nums = vec![1, 5, 1, 4, 2];
    let ret = transform_array(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 3, 2, 1];
        let expected = vec![0, 0, 1, 1];
        let ret = transform_array(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 5, 1, 4, 2];
        let expected = vec![0, 0, 1, 1, 1];
        let ret = transform_array(nums);
        assert_eq!(ret, expected);
    }
}
