fn largest_altitude(gain: Vec<i32>) -> i32 {
    gain.into_iter()
        .fold((0, 0), |(acc, ret), n| {
            let v = acc + n;
            (v, std::cmp::max(ret, v))
        })
        .1
}
fn main() {
    let ret = largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
}
