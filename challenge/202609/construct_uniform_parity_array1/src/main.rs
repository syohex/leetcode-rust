fn uniform_array(_nums1: Vec<i32>) -> bool {
    true
}

fn main() {
    let ret = uniform_array(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(uniform_array(vec![2, 3]));
    assert!(uniform_array(vec![4, 6]));
}
