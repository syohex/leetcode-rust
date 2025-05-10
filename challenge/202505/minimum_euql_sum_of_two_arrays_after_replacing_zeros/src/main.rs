fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let (sums1, zeros1) = nums1
        .into_iter()
        .fold((0i64, 0), |(mut sums, mut zeros), n| {
            sums += n as i64;
            if n == 0 {
                sums += 1i64;
                zeros += 1;
            }
            (sums, zeros)
        });
    let (sums2, zeros2) = nums2
        .into_iter()
        .fold((0i64, 0), |(mut sums, mut zeros), n| {
            sums += n as i64;
            if n == 0 {
                sums += 1i64;
                zeros += 1;
            }
            (sums, zeros)
        });

    if (zeros1 == 0 && sums1 < sums2) || (zeros2 == 0 && sums2 < sums1) {
        -1
    } else {
        std::cmp::max(sums1, sums2)
    }
}

fn main() {
    let nums1 = vec![3, 2, 0, 1, 0];
    let nums2 = vec![6, 5, 0];
    let ret = min_sum(nums1, nums2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums1 = vec![3, 2, 0, 1, 0];
        let nums2 = vec![6, 5, 0];
        let ret = min_sum(nums1, nums2);
        assert_eq!(ret, 12);
    }
    {
        let nums1 = vec![2, 0, 2, 0];
        let nums2 = vec![1, 4];
        let ret = min_sum(nums1, nums2);
        assert_eq!(ret, -1);
    }
}
