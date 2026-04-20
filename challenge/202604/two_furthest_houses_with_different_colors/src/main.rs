fn max_distance(colors: Vec<i32>) -> i32 {
    let len = colors.len();
    let mut ret = i32::MIN;
    for i in 0..len {
        for j in (i + 1)..len {
            if colors[i] != colors[j] {
                ret = std::cmp::max(ret, (j - i) as i32);
            }
        }
    }
    ret
}

fn main() {
    let ret = max_distance(vec![1, 1, 1, 6, 1, 1, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
    assert_eq!(max_distance(vec![1, 8, 3, 8, 3]), 4);
    assert_eq!(max_distance(vec![0, 1]), 1);
}
