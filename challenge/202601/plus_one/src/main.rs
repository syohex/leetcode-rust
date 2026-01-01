fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    let mut carry = 1;

    for n in digits.into_iter().rev() {
        if n + carry == 10 {
            ret.push(0);
            carry = 1;
        } else {
            ret.push(n + carry);
            carry = 0;
        }
    }

    if carry != 0 {
        ret.push(1);
    }

    ret.into_iter().rev().collect()
}

fn main() {
    let ret = plus_one(vec![9, 9, 9, 9]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(plus_one(vec![1, 2, 3]), [1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), [4, 3, 2, 2]);
    assert_eq!(plus_one(vec![9]), [1, 0]);
}
