fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    use std::collections::HashMap;

    let min_rank = *ranks.iter().min().unwrap() as i64;

    let mut h = HashMap::new();
    for &rank in &ranks {
        *h.entry(rank as i64).or_insert(0i64) += 1i64;
    }

    let cars = cars as i64;
    let mut left = 0;
    let mut right = min_rank * cars * cars;

    while left < right {
        let mid = left + (right - left) / 2;
        let can_fix_cars = h.iter().fold(0i64, |acc, (k, v)| {
            acc + v * ((mid / k) as f64).sqrt() as i64
        });

        if can_fix_cars >= cars {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}
fn main() {
    let ranks = vec![4, 3, 2, 1];
    let ret = repair_cars(ranks, 10);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let ranks = vec![4, 3, 2, 1];
        let ret = repair_cars(ranks, 10);
        assert_eq!(ret, 16);
    }
    {
        let ranks = vec![5, 1, 8];
        let ret = repair_cars(ranks, 6);
        assert_eq!(ret, 16);
    }
}
