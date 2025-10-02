fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut ret = num_bottles;
    let mut empties = num_bottles;
    let mut num_exchange = num_exchange;

    while empties >= num_exchange {
        empties -= num_exchange - 1;
        ret += 1;
        num_exchange += 1;
    }

    ret
}

fn main() {
    let ret = max_bottles_drunk(10, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_bottles_drunk(13, 6), 15);
    assert_eq!(max_bottles_drunk(10, 3), 13);
}
