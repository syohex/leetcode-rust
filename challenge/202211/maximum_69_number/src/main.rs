fn maximum69_number(num: i32) -> i32 {
    let mut num = num;
    let mut v: Vec<i32> = vec![];

    while num > 0 {
        v.push(num % 10);
        num /= 10;
    }

    v.into_iter()
        .rev()
        .fold((0, false), |(sum, ok), n| {
            if ok || n == 9 {
                (sum * 10 + n, ok)
            } else {
                (sum * 10 + 9, true)
            }
        })
        .0
}

fn main() {
    let num = 9669;
    let ret = maximum69_number(num);
    println!("ret={ret}");
}

#[test]
fn test_maximum69_number() {
    {
        let num = 9669;
        let ret = maximum69_number(num);
        assert_eq!(ret, 9969);
    }
    {
        let num = 9996;
        let ret = maximum69_number(num);
        assert_eq!(ret, 9999);
    }
    {
        let num = 9999;
        let ret = maximum69_number(num);
        assert_eq!(ret, 9999);
    }
    {
        let num = 6;
        let ret = maximum69_number(num);
        assert_eq!(ret, 9);
    }
}
