fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    fn f(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if m <= 0 && n <= 0 {
            return;
        }

        let index = (m + n - 1) as usize;
        match (m > 0, n > 0) {
            (true, true) => {
                if nums1[(m - 1) as usize] >= nums2[(n - 1) as usize] {
                    nums1[index] = nums1[(m - 1) as usize];
                    f(nums1, m - 1, nums2, n);
                } else {
                    nums1[index] = nums2[(n - 1) as usize];
                    f(nums1, m, nums2, n - 1);
                }
            }
            (true, false) => {
                nums1[index] = nums1[(m - 1) as usize];
                f(nums1, m - 1, nums2, n);
            }
            (false, true) => {
                nums1[index] = nums2[(n - 1) as usize];
                f(nums1, m, nums2, n - 1);
            }
            _ => {
                panic!("never reach here: m={m}, n={n}");
            }
        }
    }

    f(nums1, m, nums2, n);
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("ret={:?}", nums1);
}

#[test]
fn test_merge() {
    {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let expected = vec![1, 2, 2, 3, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, expected);
    }
    {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let expected = vec![1];
        merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, expected);
    }
    {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let expected = vec![1];
        merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, expected);
    }
}
