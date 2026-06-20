fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
    fn f(
        i: i32,
        cost: i32,
        n: i32,
        k: i32,
        is_prev_one: bool,
        acc: &mut String,
        ret: &mut Vec<String>,
    ) {
        if cost > k {
            return;
        }

        if i == n {
            ret.push(acc.clone());
            return;
        }

        acc.push('0');
        f(i + 1, cost, n, k, false, acc, ret);
        acc.pop();

        if !is_prev_one {
            acc.push('1');
            f(i + 1, cost + i, n, k, true, acc, ret);
            acc.pop();
        }
    }

    let mut ret = vec![];
    let mut acc = String::new();
    f(0, 0, n, k, false, &mut acc, &mut ret);
    ret
}

fn main() {
    let ret = generate_valid_strings(3, 1);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        generate_valid_strings(3, 1),
        vec!["000".to_string(), "010".to_string(), "100".to_string()],
    );
    assert_eq!(
        generate_valid_strings(1, 0),
        vec!["0".to_string(), "1".to_string()],
    );
}
