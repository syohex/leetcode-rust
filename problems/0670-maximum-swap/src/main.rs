fn maximum_swap(num: i32) -> i32 {
    let mut nums = vec![];
    let mut num = num;

    if num == 0 {
        nums.push(0);
    } else {
        while num > 0 {
            nums.push(num % 10);
            num /= 10;
        }

        nums.reverse();
    }

    let len = nums.len();
    for i in 0..(len - 1) {
        let mut index = i;
        let mut max = nums[i];
        for j in (i + 1)..len {
            if max <= nums[j] {
                index = j;
                max = nums[j];
            }
        }

        if nums[i] != nums[index] {
            nums.swap(i, index);
            break;
        }
    }

    nums.into_iter().fold(0, |acc, n| acc * 10 + n)
}

fn main() {
    let ret = maximum_swap(2736);
    println!("ret={ret}");
}

#[test]
fn test_maximum_swap() {
    assert_eq!(maximum_swap(2736), 7236);
    assert_eq!(maximum_swap(9973), 9973);
    assert_eq!(maximum_swap(1993), 9913);
    assert_eq!(maximum_swap(22341345), 52341342);
    assert_eq!(maximum_swap(98368), 98863);
}
