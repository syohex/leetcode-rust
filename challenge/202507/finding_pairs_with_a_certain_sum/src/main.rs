use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    h: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut h = HashMap::new();
        for &num2 in &nums2 {
            *h.entry(num2).or_insert(0) += 1;
        }

        Self { nums1, nums2, h }
    }

    fn add(&mut self, index: i32, val: i32) {
        let old = self.nums2[index as usize];
        if let Some(v) = self.h.get_mut(&old) {
            *v -= 1;
        }
        self.nums2[index as usize] += val;

        let new = old + val;
        *self.h.entry(new).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut ret = 0;
        for &num1 in &self.nums1 {
            let diff = tot - num1;
            if let Some(&v) = self.h.get(&diff) {
                ret += v;
            }
        }

        ret
    }
}

fn main() {
    let nums1 = vec![1, 1, 2, 2, 2, 3];
    let nums2 = vec![1, 4, 5, 2, 5, 4];
    let mut f = FindSumPairs::new(nums1, nums2);
    f.add(3, 2);
    let ret = f.count(8);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums1 = vec![1, 1, 2, 2, 2, 3];
        let nums2 = vec![1, 4, 5, 2, 5, 4];
        let mut f = FindSumPairs::new(nums1, nums2);
        assert_eq!(f.count(7), 8);
        f.add(3, 2);
        assert_eq!(f.count(8), 2);
        assert_eq!(f.count(4), 1);
        f.add(0, 1);
        f.add(1, 1);
        assert_eq!(f.count(7), 11);
    }
}
