fn broken_calc(start_value: i32, target: i32) -> i32 {
    let mut target = target;
    let mut count = 0;

    while start_value < target {
        count += 1;
        if target % 2 == 1 {
            target += 1;
        } else {
            target /= 2;
        }
    }

    count + start_value - target
}

fn main() {
    let ret = broken_calc(3, 10);
    println!("ret={ret}");
}

#[test]
fn test_broken_calc() {
    assert_eq!(broken_calc(2, 3), 2);
    assert_eq!(broken_calc(5, 8), 2);
    assert_eq!(broken_calc(3, 10), 3);
}
