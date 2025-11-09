fn count_operations(num1: i32, num2: i32) -> i32 {
    fn f(n1: i32, n2: i32, acc: i32) -> i32 {
        if n1 == 0 || n2 == 0 {
            acc
        } else if n1 >= n2 {
            f(n1 - n2, n2, acc + 1)
        } else {
            f(n1, n2 - n1, acc + 1)
        }
    }

    f(num1, num2, 0)
}

fn main() {
    let ret = count_operations(2, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_operations(2, 3), 3);
    assert_eq!(count_operations(10, 10), 1);
}
