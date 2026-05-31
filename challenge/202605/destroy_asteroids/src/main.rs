fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
    let mut asteroids = asteroids;
    asteroids.sort_unstable();

    let mut sum = mass as i64;
    for asteroid in asteroids {
        let a = asteroid as i64;
        if sum < a {
            return false;
        }

        sum += a;
    }

    true
}

fn main() {
    let ret = asteroids_destroyed(10, vec![3, 9, 19, 5, 21]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(asteroids_destroyed(10, vec![3, 9, 19, 5, 21]));
    assert!(!asteroids_destroyed(5, vec![4, 9, 23, 4]));
}
