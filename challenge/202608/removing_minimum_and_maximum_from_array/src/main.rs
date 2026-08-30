fn removing_minimum_and_maximum_from_array(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut min_index = 0;
    let mut max = i32::MIN;
    let mut max_index = 0;
    let len = nums.len();

    for (i, n) in nums.into_iter().enumerate() {
        if n < min {
            min = n;
            min_index = i;
        }
        if n > max {
            max = n;
            max_index = i;
        }
    }

    let from_front = std::cmp::max(min_index, max_index) + 1;
    let from_back = len - std::cmp::min(min_index, max_index);
    let front_and_back = if min_index < max_index {
        min_index + 1 + len - max_index
    } else {
        max_index + 1 + len - min_index
    };

    std::cmp::min(from_front, std::cmp::min(from_back, front_and_back)) as i32
}

fn main() {
    let ret = removing_minimum_and_maximum_from_array(vec![2, 10, 7, 5, 4, 1, 8, 6]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        removing_minimum_and_maximum_from_array(vec![2, 10, 7, 5, 4, 1, 8, 6]),
        5
    );
    assert_eq!(
        removing_minimum_and_maximum_from_array(vec![0, -4, 19, 1, 8, -2, -3, 5]),
        3
    );
    assert_eq!(removing_minimum_and_maximum_from_array(vec![101]), 1);
}
