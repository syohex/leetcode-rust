fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![nums[0]];
    let mut arr2 = vec![nums[1]];

    for n in nums.into_iter().skip(2) {
        if arr1[arr1.len() - 1] > arr2[arr2.len() - 1] {
            arr1.push(n);
        } else {
            arr2.push(n);
        }
    }

    arr1.append(&mut arr2);

    arr1
}

fn main() {
    let ret = result_array(vec![2, 1, 3]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(result_array(vec![2, 1, 3]), vec![2, 3, 1]);
    assert_eq!(result_array(vec![5, 4, 3, 8]), vec![5, 3, 4, 8]);
}
