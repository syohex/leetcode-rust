fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for trip in trips.iter() {
        *h.entry(trip[1]).or_insert(0) += trip[0];
        *h.entry(trip[2]).or_insert(0) -= trip[0];
    }

    let mut sorted: Vec<(i32, i32)> = h.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    let mut cap = 0;
    for (_, v) in sorted.into_iter() {
        cap += v;
        if cap > capacity {
            return false;
        }
    }

    true
}

fn main() {
    let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
    println!("ret={}", car_pooling(trips, 5));
}

#[test]
fn test_car_pooling() {
    {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        assert!(!car_pooling(trips, 4));
    }
    {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        assert!(car_pooling(trips, 5));
    }
}
