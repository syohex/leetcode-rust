fn average(salary: Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;
    let len = salary.len();

    for s in salary {
        min = std::cmp::min(min, s);
        max = std::cmp::max(max, s);
        sum += s;
    }

    ((sum - min - max) as f64) / ((len - 2) as f64)
}

fn main() {
    let salary = vec![4000, 3000, 1000, 2000];
    let ret = average(salary);
    println!("ret={ret}");
}

#[test]
fn test_average() {
    {
        let salary = vec![4000, 3000, 1000, 2000];
        let ret = average(salary);
        assert_eq!(ret, 2500.0);
    }
    {
        let salary = vec![1000, 2000, 3000];
        let ret = average(salary);
        assert_eq!(ret, 2000.0);
    }
}
