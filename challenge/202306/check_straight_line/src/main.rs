fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    let diff_x = coordinates[1][0] - coordinates[0][0];
    let diff_y = coordinates[1][1] - coordinates[0][1];

    if diff_x == 0 {
        coordinates.iter().skip(1).all(|v| {
            v[0] == coordinates[0][0]
        })
    } else {
        coordinates.iter().skip(2).all(|v| {
            diff_x * (v[1] - coordinates[0][1]) == diff_y * (v[0] - coordinates[0][0])
        })
    }
}

fn main() {
    let coordinates = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![6, 7],
    ];
    println!("ret={}", check_straight_line(coordinates));
}

#[test]
fn test_check_straight_line() {
    {
        let coordinates = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7],
        ];
        assert!(check_straight_line(coordinates));
    }
    {
        let coordinates = vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7],
        ];
        assert!(!check_straight_line(coordinates));
    }
    {
        let coordinates = vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, -1],
        ];
        assert!(check_straight_line(coordinates));
    }
}
