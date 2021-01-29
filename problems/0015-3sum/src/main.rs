fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = vec![];
    let mut sorted: Vec<i32> = nums.into_iter().collect();
    sorted.sort_unstable();

    for (i, &num) in sorted.iter().enumerate() {
        if i > 0 && num == sorted[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = sorted.len() - 1;
        while left < right {
            let sum = num + sorted[left] + sorted[right];
            if sum == 0 {
                ret.push(vec![num, sorted[left], sorted[right]]);

                let prev = sorted[left];
                left += 1;
                while left < sorted.len() && prev == sorted[left] {
                    left += 1;
                }

                let prev = sorted[right];
                right -= 1;
                while left < right && prev == sorted[right] {
                    right -= 1;
                }
            } else if sum > 0 {
                right -= 1;
            } else {
                left += 1;
            }
        }
    }
    ret
}

fn main() {
    let input: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
    let ret = three_sum(input.clone());
    println!("three_sum({:?}) = {:?}", input, ret);
}

#[test]
fn test_three_sum() {
    {
        let input: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
        let expected: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(three_sum(input), expected);
    }
    {
        let input: Vec<i32> = vec![];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(input), expected);
    }
    {
        let input: Vec<i32> = vec![0];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(input), expected);
    }
}
