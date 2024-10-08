fn min_swaps(s: String) -> i32 {
    let mut unbalanced = 0;
    let mut open_brackets = 0;

    for c in s.chars() {
        if c == '[' {
            open_brackets += 1;
        } else {
            if open_brackets > 0 {
                open_brackets -= 1;
            } else {
                unbalanced += 1;
            }
        }
    }

    if unbalanced % 2 == 0 {
        unbalanced / 2
    } else {
        unbalanced / 2 + 1
    }
}

fn main() {
    let ret = min_swaps("]]][[[".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_swaps("][][".to_string()), 1);
    assert_eq!(min_swaps("]]][[[".to_string()), 2);
    assert_eq!(min_swaps("[]".to_string()), 0);
}
