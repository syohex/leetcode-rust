fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = *candies.iter().max().unwrap();
    candies
        .into_iter()
        .map(|n| n + extra_candies >= max)
        .collect()
}

fn main() {
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let ret = kids_with_candies(candies, extra_candies);
    println!("ret={ret:?}");
}

#[test]
fn test_kids_with_candies() {
    {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let expected = vec![true, true, true, false, true];
        let ret = kids_with_candies(candies, extra_candies);
        assert_eq!(ret, expected);
    }
    {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        let expected = vec![true, false, false, false, false];
        let ret = kids_with_candies(candies, extra_candies);
        assert_eq!(ret, expected);
    }
    {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let expected = vec![true, false, true];
        let ret = kids_with_candies(candies, extra_candies);
        assert_eq!(ret, expected);
    }
}
