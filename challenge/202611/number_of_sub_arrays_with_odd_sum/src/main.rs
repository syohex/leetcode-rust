fn num_of_arrays(arr: Vec<i32>) -> i32 {
    let modulo = 1_000_000_007;
    let mut acc = 0;
    let mut odds = 0;
    let mut evens = 1;

    let mut ret = 0;
    for n in arr {
        acc += n;
        if acc % 2 == 0 {
            ret = (ret + odds) % modulo;
            evens += 1;
        } else {
            ret = (ret + evens) % modulo;
            odds += 1;
        }
    }

    ret
}

fn main() {
    let ret = num_of_arrays(vec![1, 2, 3, 4, 5, 6, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(num_of_arrays(vec![1, 3, 5]), 4);
    assert_eq!(num_of_arrays(vec![2, 4, 6]), 0);
    assert_eq!(num_of_arrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
}
