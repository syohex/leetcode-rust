fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut num_bottles = num_bottles;
    let mut empties = 0;

    let mut ret = 0;
    while num_bottles > 0 {
        ret += num_bottles;

        let (d, m) = (num_bottles / num_exchange, num_bottles % num_exchange);
        num_bottles = d;
        empties += m;

        if empties >= num_exchange {
            num_bottles += empties / num_exchange;
            empties %= num_exchange;
        }
    }

    ret
}

fn main() {
    let ret = num_water_bottles(15, 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(num_water_bottles(9, 3), 13);
    assert_eq!(num_water_bottles(15, 4), 19);
    assert_eq!(num_water_bottles(25, 4), 33);
}
