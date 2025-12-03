fn minimum_distance(nums: Vec<i32>) -> i32 {
    let mut positions = vec![vec![]; nums.len() + 1];

    for (i, n) in nums.into_iter().enumerate() {
        positions[n as usize].push(i);
    }

    let mut ret = i32::MAX;
    for p in positions {
        let len = p.len();
        for i in 0..len {
            for j in (i + 1)..len {
                for k in (j + 1)..len {
                    ret = std::cmp::min(ret, (p[j] - p[i] + p[k] - p[i] + p[k] - p[j]) as i32);
                }
            }
        }
    }

    if ret == i32::MAX {
        -1
    } else {
        ret
    }
}

fn main() {
    let nums = vec![1, 2, 1, 1, 3];
    let ret = minimum_distance(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 1, 1, 3];
        let ret = minimum_distance(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![1, 1, 2, 3, 2, 1, 2];
        let ret = minimum_distance(nums);
        assert_eq!(ret, 8);
    }
    {
        let nums = vec![1];
        let ret = minimum_distance(nums);
        assert_eq!(ret, -1);
    }
}
