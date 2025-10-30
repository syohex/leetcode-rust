fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut ret = target[0];
    for i in 1..target.len() {
        ret += std::cmp::max(target[i] - target[i - 1], 0);
    }

    ret
}

fn main() {
    let target = vec![1, 2, 3, 2, 1];
    let ret = min_number_operations(target);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let target = vec![1, 2, 3, 2, 1];
        let ret = min_number_operations(target);
        assert_eq!(ret, 3);
    }
    {
        let target = vec![3, 1, 1, 2];
        let ret = min_number_operations(target);
        assert_eq!(ret, 4);
    }
    {
        let target = vec![3, 1, 5, 4, 2];
        let ret = min_number_operations(target);
        assert_eq!(ret, 7);
    }
}
