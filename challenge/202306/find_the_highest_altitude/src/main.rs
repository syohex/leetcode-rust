fn largest_altitude(gain: Vec<i32>) -> i32 {
    use std::cmp::max;

    gain.into_iter()
        .fold((0, 0), |(alt, ret), n| {
            let alt = alt + n;
            (alt, max(ret, alt))
        })
        .1
}
fn main() {
    let gain = vec![-5, 1, 5, 0, -7];
    let ret = largest_altitude(gain);
    println!("ret={ret}");
}

#[test]
fn test_find_the_highest_altitude() {
    {
        let gain = vec![-5, 1, 5, 0, -7];
        let ret = largest_altitude(gain);
        assert_eq!(ret, 1);
    }
    {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let ret = largest_altitude(gain);
        assert_eq!(ret, 0);
    }
}
