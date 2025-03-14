fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let mut left = 0;
    let mut right = *candies.iter().max().unwrap();

    while left < right {
        let mid = (left + right + 1) / 2;
        let ok = candies
            .iter()
            .fold(0i64, |acc, &candy| acc + (candy / mid) as i64)
            >= k;
        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn main() {
    let candies = vec![5, 8, 6];
    let ret = maximum_candies(candies, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let candies = vec![1, 2, 3, 4, 10];
        let ret = maximum_candies(candies, 5);
        assert_eq!(ret, 3);
    }
    {
        let candies = vec![5, 8, 6];
        let ret = maximum_candies(candies, 3);
        assert_eq!(ret, 5);
    }
    {
        let candies = vec![2, 5];
        let ret = maximum_candies(candies, 11);
        assert_eq!(ret, 0);
    }
}
