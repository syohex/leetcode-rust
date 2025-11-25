fn smallest_repunit_div_by_k(k: i32) -> i32 {
    use std::collections::HashSet;
    let mut sum = 0;
    let mut digits = 1;

    let mut checked = HashSet::new();
    loop {
        sum = (sum * 10 + 1) % k;
        if sum == 0 {
            return digits;
        }
        if checked.contains(&sum) {
            return -1;
        }

        checked.insert(sum);
        digits += 1;
    }
}

fn main() {
    let ret = smallest_repunit_div_by_k(3928);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(smallest_repunit_div_by_k(1), 1);
    assert_eq!(smallest_repunit_div_by_k(2), -1);
    assert_eq!(smallest_repunit_div_by_k(3), 3);
}
