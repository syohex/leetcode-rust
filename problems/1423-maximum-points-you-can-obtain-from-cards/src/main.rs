fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let revs: Vec<i32> = card_points
        .iter()
        .skip(card_points.len() - k as usize)
        .copied()
        .collect();
    let mut sum: i32 = revs.iter().sum();
    let mut max = sum;
    for (&i, &j) in card_points.iter().take(k as usize).zip(&revs) {
        sum += i;
        sum -= j;

        if sum > max {
            max = sum;
        }
    }

    max
}

fn main() {
    let ret = max_score(vec![1, 2, 3, 4, 5, 6, 1], 3);
    println!("ret={}", ret);
}

#[test]
fn test_max_score() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
    assert_eq!(max_score(vec![1, 1000, 1], 1), 1);
    assert_eq!(max_score(vec![100, 40, 17, 9, 73, 75], 3), 248);
}
