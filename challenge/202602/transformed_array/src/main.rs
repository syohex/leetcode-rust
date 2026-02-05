fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .enumerate()
        .map(|(i, &n)| {
            if n >= 0 {
                nums[(i + n as usize) % nums.len()]
            } else {
                let j = n % nums.len() as i32;
                nums[(i as i32 + nums.len() as i32 + j) as usize % nums.len()]
            }
        })
        .collect()
}

fn main() {
    let ret = construct_transformed_array(vec![3, -2, 1, 1]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        construct_transformed_array(vec![-10, -10, -4]),
        [-4, -10, -10]
    );
    assert_eq!(construct_transformed_array(vec![3, -2, 1, 1]), [1, 1, 1, 3]);
    assert_eq!(construct_transformed_array(vec![-1, 4, -1]), [-1, -1, 4]);
}
