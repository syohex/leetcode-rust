fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;

    let mut i1 = 0;
    let mut i2 = 0;

    let mut ret = vec![];
    while i1 < nums1.len() || i2 < nums2.len() {
        if i1 == nums1.len() {
            ret.push(nums2[i2].clone());
            i2 += 1;
            continue;
        }
        if i2 == nums2.len() {
            ret.push(nums1[i1].clone());
            i1 += 1;
            continue;
        }

        match nums1[i1][0].cmp(&nums2[i2][0]) {
            Ordering::Equal => {
                ret.push(vec![nums1[i1][0], nums1[i1][1] + nums2[i2][1]]);
                i1 += 1;
                i2 += 1;
            }
            Ordering::Less => {
                ret.push(nums1[i1].clone());
                i1 += 1;
            }
            Ordering::Greater => {
                ret.push(nums2[i2].clone());
                i2 += 1;
            }
        }
    }

    ret
}

fn main() {
    let nums1 = vec![vec![2, 4], vec![3, 6], vec![5, 5]];
    let nums2 = vec![vec![1, 3], vec![4, 3]];
    let ret = merge_arrays(nums1, nums2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums1 = vec![vec![1, 2], vec![2, 3], vec![4, 5]];
        let nums2 = vec![vec![1, 4], vec![3, 2], vec![4, 1]];
        let expected = vec![vec![1, 6], vec![2, 3], vec![3, 2], vec![4, 6]];
        let ret = merge_arrays(nums1, nums2);
        assert_eq!(ret, expected);
    }
    {
        let nums1 = vec![vec![2, 4], vec![3, 6], vec![5, 5]];
        let nums2 = vec![vec![1, 3], vec![4, 3]];
        let expected = vec![vec![1, 3], vec![2, 4], vec![3, 6], vec![4, 3], vec![5, 5]];
        let ret = merge_arrays(nums1, nums2);
        assert_eq!(ret, expected);
    }
}
