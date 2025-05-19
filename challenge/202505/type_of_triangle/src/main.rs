fn triangle_type(nums: Vec<i32>) -> String {
    if !(nums[0] + nums[1] > nums[2] && nums[1] + nums[2] > nums[0] && nums[0] + nums[2] > nums[1])
    {
        "none".to_string()
    } else if nums[0] == nums[1] && nums[1] == nums[2] {
        "equilateral".to_string()
    } else if nums[0] == nums[1] || nums[1] == nums[2] || nums[0] == nums[2] {
        "isosceles".to_string()
    } else {
        "scalene".to_string()
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let ret = triangle_type(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 3, 3];
        let ret = triangle_type(nums);
        assert_eq!(ret, "equilateral");
    }
    {
        let nums = vec![3, 4, 5];
        let ret = triangle_type(nums);
        assert_eq!(ret, "scalene");
    }
    {
        let nums = vec![8, 4, 4];
        let ret = triangle_type(nums);
        assert_eq!(ret, "none");
    }
}
