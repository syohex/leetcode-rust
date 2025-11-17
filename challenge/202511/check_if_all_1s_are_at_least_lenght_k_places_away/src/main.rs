fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let len = nums.len();
    let mut prev = len;
    for (i, n) in nums.into_iter().enumerate() {
        if n == 1 {
            if prev != len && i - prev - 1 < k {
                return false;
            }

            prev = i;
        }
    }

    true
}

fn main() {
    let ret = k_length_apart(vec![1,0,0,0,1,0,0,1], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!k_length_apart(vec![1,0,0,1,0,1], 2));
    assert!(k_length_apart(vec![1,0,0,0,1,0,0,1], 2));
}
