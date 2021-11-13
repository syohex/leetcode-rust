fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![0; temperatures.len()];
    let mut stack: Vec<usize> = vec![];

    for (i, &temp) in temperatures.iter().enumerate() {
        while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temp {
            let j = stack.pop().unwrap();
            ret[j] = (i - j) as i32;
        }

        stack.push(i);
    }

    while !stack.is_empty() {
        let index = stack.pop().unwrap();
        ret[index] = 0;
    }

    ret
}

fn main() {
    let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let ret = daily_temperatures(temperatures);
    println!("ret={:?}", ret);
}

#[test]
fn test_daily_temperatures() {
    {
        let temperatures = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let expected = vec![1, 1, 4, 2, 1, 1, 0, 0];
        let ret = daily_temperatures(temperatures);
        assert_eq!(ret, expected);
    }
    {
        let temperatures = vec![30, 40, 50, 60];
        let expected = vec![1, 1, 1, 0];
        let ret = daily_temperatures(temperatures);
        assert_eq!(ret, expected);
    }
    {
        let temperatures = vec![30, 60, 90];
        let expected = vec![1, 1, 0];
        let ret = daily_temperatures(temperatures);
        assert_eq!(ret, expected);
    }
}
