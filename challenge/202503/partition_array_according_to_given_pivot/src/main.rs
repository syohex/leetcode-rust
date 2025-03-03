fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut smallers = vec![];
    let mut pivots = vec![];
    let mut largers = vec![];

    for num in nums {
        match num.cmp(&pivot) {
            std::cmp::Ordering::Less => {
                smallers.push(num);
            }
            std::cmp::Ordering::Equal => {
                pivots.push(num);
            }
            std::cmp::Ordering::Greater => {
                largers.push(num);
            }
        }
    }

    let mut ret = smallers;
    for n in pivots {
        ret.push(n);
    }
    for n in largers {
        ret.push(n);
    }
    ret
}

fn main() {
    let nums = vec![9, 12, 5, 10, 14, 3, 10];
    let pivot = 10;
    let ret = pivot_array(nums, pivot);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![9, 12, 5, 10, 14, 3, 10];
        let pivot = 10;
        let expected = vec![9, 5, 3, 10, 10, 12, 14];
        let ret = pivot_array(nums, pivot);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![-3, 4, 3, 2];
        let pivot = 2;
        let expected = vec![-3, 2, 4, 3];
        let ret = pivot_array(nums, pivot);
        assert_eq!(ret, expected);
    }
}
