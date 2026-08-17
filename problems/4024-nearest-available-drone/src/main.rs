fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut min_dist = i32::MAX;

    for (i, d) in drones.into_iter().enumerate() {
        let dist = (d[0] - target[0]).abs() + (d[1] - target[1]).abs();
        if dist <= d[2] && dist < min_dist {
            min_dist = dist;
            ret = i;
        }
    }

    if min_dist == i32::MAX { -1 } else { ret as i32 }
}

fn main() {
    let ret = nearest_drone(vec![vec![0, 0, 8], vec![2, 2, 3]], vec![3, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        nearest_drone(vec![vec![0, 0, 8], vec![2, 2, 3]], vec![3, 4]),
        1
    );
    assert_eq!(
        nearest_drone(
            vec![vec![2, 1, 5], vec![4, 4, 5], vec![6, 6, 8]],
            vec![5, 5]
        ),
        1
    );
    assert_eq!(nearest_drone(vec![vec![4, 4, 5]], vec![8, 6]), -1);
}
