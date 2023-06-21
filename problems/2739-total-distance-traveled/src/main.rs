fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
    let mut main_tank = main_tank;
    let mut additional_tank = additional_tank;

    let mut ret = 0;

    loop {
        if main_tank < 5 {
            ret += 10 * main_tank;
            return ret;
        }

        ret += 50;
        main_tank -= 5;

        if additional_tank > 0 {
            main_tank += 1;
            additional_tank -= 1;
        }
    }
}

fn main() {
    let ret = distance_traveled(5, 10);
    println!("ret={ret}");
}

#[test]
fn test_distance_traveled() {
    {
        let ret = distance_traveled(5, 10);
        assert_eq!(ret, 60);
    }
    {
        let ret = distance_traveled(1, 2);
        assert_eq!(ret, 10);
    }
    {
        let ret = distance_traveled(10, 1);
        assert_eq!(ret, 110);
    }
}
