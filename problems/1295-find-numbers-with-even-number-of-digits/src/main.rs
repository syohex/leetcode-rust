fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.iter()
        .map(|x| x.to_string())
        .filter(|x| x.len() % 2 == 0)
        .count() as i32
}

fn main() {
    println!(
        "find_numbers([12, 345, 2, 6, 7896]) = {}",
        find_numbers(vec![12, 345, 2, 6, 7896])
    );
}

#[test]
fn test_find_numbers() {
    assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
}
