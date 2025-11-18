fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut i = 0;

    while i < bits.len() {
        if i + 1 == bits.len() {
            return bits[i] == 0;
        }

        if bits[i] == 0 {
            i += 1;
        } else {
            i += 2;
        }
    }

    false
}

fn main() {
    let ret = is_one_bit_character(vec![1, 1, 1, 0]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_one_bit_character(vec![1, 0, 0]));
    assert!(!is_one_bit_character(vec![1, 1, 1, 0]));
}
