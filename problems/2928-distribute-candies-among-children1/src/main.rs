fn distribute_candies(n: i32, limit: i32) -> i32 {
    fn f(n: i32, people: i32, limit: i32) -> i32 {
        if people >= 3 {
            if n == 0 {
                return 1;
            }

            return 0;
        }

        let mut ret = 0;
        let limit = std::cmp::min(n, limit);
        for i in 0..=limit {
            ret += f(n - i, people + 1, limit);
        }

        ret
    }

    f(n, 0, limit)
}

fn main() {
    let ret = distribute_candies(3, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(distribute_candies(5, 2), 3);
    assert_eq!(distribute_candies(3, 3), 10);
}
