fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    fn f(pos1: usize, pos2: usize, nums1: &[i32], nums2: &[i32], dp: &mut Vec<Vec<i32>>) -> i32 {
        if pos1 == nums1.len() || pos2 == nums2.len() {
            return -1_000_000_0;
        }

        if dp[pos1][pos2] != i32::MIN {
            dp[pos1][pos2]
        } else {
            let v0 = nums1[pos1] * nums2[pos2];
            let v1 = v0 + f(pos1 + 1, pos2 + 1, nums1, nums2, dp);
            let v2 = f(pos1 + 1, pos2, nums1, nums2, dp);
            let v3 = f(pos1, pos2 + 1, nums1, nums2, dp);

            let v = std::cmp::max(std::cmp::max(v0, v1), std::cmp::max(v2, v3));
            dp[pos1][pos2] = v;
            v
        }
    }

    let mut dp = vec![vec![i32::MIN; nums2.len()]; nums1.len()];
    f(0, 0, &nums1, &nums2, &mut dp)
}

fn main() {
    let nums1 = vec![2, 1, -2, 5];
    let nums2 = vec![3, 0, -6];
    let ret = max_dot_product(nums1, nums2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums1 = vec![-5, -1, -2];
        let nums2 = vec![3, 3, 5, 5];
        let ret = max_dot_product(nums1, nums2);
        assert_eq!(ret, -3);
    }
    {
        let nums1 = vec![2, 1, -2, 5];
        let nums2 = vec![3, 0, -6];
        let ret = max_dot_product(nums1, nums2);
        assert_eq!(ret, 18);
    }
    {
        let nums1 = vec![3, -2];
        let nums2 = vec![2, -6, 7];
        let ret = max_dot_product(nums1, nums2);
        assert_eq!(ret, 21);
    }
    {
        let nums1 = vec![-1, -1];
        let nums2 = vec![1, 1];
        let ret = max_dot_product(nums1, nums2);
        assert_eq!(ret, -1);
    }
}
