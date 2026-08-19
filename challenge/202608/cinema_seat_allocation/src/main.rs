fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let mut reserved_rows = HashMap::new();
    for seat in reserved_seats {
        if seat[1] >= 2 && seat[1] <= 9 {
            *reserved_rows.entry(seat[0]).or_insert(0) |= 1 << (seat[1] - 2);
        }
    }

    let pattern1 = 0b00001111;
    let pattern2 = 0b11000011;
    let pattern3 = 0b11110000;
    let mut ret = (n - reserved_rows.len() as i32) * 2;
    for v in reserved_rows.into_values() {
        if ((v | pattern1) == pattern1)
            || ((v | pattern2) == pattern2)
            || ((v | pattern3) == pattern3)
        {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let n = 3;
    let reserved_seats = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 8],
        vec![2, 6],
        vec![3, 1],
        vec![3, 10],
    ];
    let ret = max_number_of_families(n, reserved_seats);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 3;
        let reserved_seats = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 8],
            vec![2, 6],
            vec![3, 1],
            vec![3, 10],
        ];
        let ret = max_number_of_families(n, reserved_seats);
        assert_eq!(ret, 4);
    }

    {
        let n = 2;
        let reserved_seats = vec![vec![2, 1], vec![1, 8], vec![2, 6]];
        let ret = max_number_of_families(n, reserved_seats);
        assert_eq!(ret, 2);
    }
    {
        let n = 4;
        let reserved_seats = vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]];
        let ret = max_number_of_families(n, reserved_seats);
        assert_eq!(ret, 4);
    }
}
