fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
    let mut happiness = happiness;
    happiness.sort_by_key(|v| std::cmp::Reverse(*v));

    let mut ret = 0;
    let mut turns = 0;
    for h in happiness.into_iter().take(k as usize) {
        let v = (h - turns) as i64;
        if v <= 0 {
            break;
        }

        ret += v;
        turns += 1;
    }

    ret
}

fn main() {
    let happiness = vec![1, 2, 3];
    let ret = maximum_happiness_sum(happiness, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let happiness = vec![1, 2, 3];
        let ret = maximum_happiness_sum(happiness, 2);
        assert_eq!(ret, 4);
    }
    {
        let happiness = vec![1, 1, 1, 1];
        let ret = maximum_happiness_sum(happiness, 2);
        assert_eq!(ret, 1);
    }
    {
        let happiness = vec![2, 3, 4, 5];
        let ret = maximum_happiness_sum(happiness, 1);
        assert_eq!(ret, 5);
    }
}
