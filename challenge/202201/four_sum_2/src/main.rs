fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h: HashMap<i32, i32> = HashMap::new();
    for n1 in &nums1 {
        for n2 in &nums2 {
            *h.entry(n1 + n2).or_insert(0) += 1;
        }
    }

    let mut ret = 0;
    for n3 in &nums3 {
        for n4 in &nums4 {
            let n = -(n3 + n4);
            if let Some(m) = h.get(&n) {
                ret += m;
            }
        }
    }

    ret
}

fn main() {
    let nums1 = vec![1, 2];
    let nums2 = vec![-2, -1];
    let nums3 = vec![-1, 2];
    let nums4 = vec![0, 2];
    let ret = four_sum_count(nums1, nums2, nums3, nums4);
    println!("ret={ret}");
}

#[test]
fn test_four_sum_count() {
    {
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];
        let ret = four_sum_count(nums1, nums2, nums3, nums4);
        assert_eq!(ret, 2);
    }
    {
        let nums1 = vec![0];
        let nums2 = vec![0];
        let nums3 = vec![0];
        let nums4 = vec![0];
        let ret = four_sum_count(nums1, nums2, nums3, nums4);
        assert_eq!(ret, 1);
    }
}
