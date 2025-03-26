fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut nums = vec![];
    for v in grid {
        for n in v {
            nums.push(n);
        }
    }

    nums.sort_unstable();

    let mid = nums[nums.len() / 2];

    let mut ret = 0;
    for n in nums {
        let diff = (mid - n).abs();
        if diff % x != 0 {
            return -1;
        }

        ret += diff / x;
    }

    ret
}

fn main() {
    let grid = vec![vec![2, 4], vec![6, 8]];
    let ret = min_operations(grid, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![2, 4], vec![6, 8]];
        let ret = min_operations(grid, 2);
        assert_eq!(ret, 4);
    }
    {
        let grid = vec![vec![1, 5], vec![2, 3]];
        let ret = min_operations(grid, 1);
        assert_eq!(ret, 5);
    }
    {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let ret = min_operations(grid, 2);
        assert_eq!(ret, -1);
    }
}
