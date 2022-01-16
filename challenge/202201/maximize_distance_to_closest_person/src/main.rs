fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    use std::cmp::max;

    let v: Vec<i32> = seats
        .iter()
        .enumerate()
        .filter(|(_, n)| **n == 1)
        .map(|(i, _)| i as i32)
        .collect();

    let mut ret = v[0];
    ret = max(ret, (seats.len() - 1) as i32 - v[v.len() - 1]);
    for i in 1..v.len() {
        let val = (v[i] - v[i - 1]) / 2;
        ret = max(ret, val);
    }

    ret
}

fn main() {
    let seats = vec![1, 0, 0, 0, 1, 0, 1];
    let ret = max_dist_to_closest(seats);
    println!("{ret}");
}

#[test]
fn test_max_dist_to_closest() {
    {
        let seats = vec![1, 0, 0, 0, 1, 0, 1];
        assert_eq!(max_dist_to_closest(seats), 2);
    }
    {
        let seats = vec![1, 0, 0, 0];
        assert_eq!(max_dist_to_closest(seats), 3);
    }
    {
        let seats = vec![0, 1];
        assert_eq!(max_dist_to_closest(seats), 1);
    }
}
