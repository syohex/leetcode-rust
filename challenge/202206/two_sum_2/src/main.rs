fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            return vec![left as i32 + 1, right as i32 + 1];
        }

        if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    panic!("never reach here");
}

fn main() {
    let numbers = vec![2, 7, 11, 15];
    let ret = two_sum(numbers, 9);
    println!("ret={:?}", ret);
}

#[test]
fn test_two_sum() {
    {
        let numbers = vec![2, 7, 11, 15];
        let expected = vec![1, 2];
        let ret = two_sum(numbers, 9);
        assert_eq!(ret, expected);
    }
    {
        let numbers = vec![2, 3, 4];
        let expected = vec![1, 3];
        let ret = two_sum(numbers, 6);
        assert_eq!(ret, expected);
    }
    {
        let numbers = vec![-1, 0];
        let expected = vec![1, 2];
        let ret = two_sum(numbers, -1);
        assert_eq!(ret, expected);
    }
}
