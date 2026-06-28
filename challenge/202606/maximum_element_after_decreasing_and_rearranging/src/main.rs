fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
    let mut arr = arr;
    arr.sort_unstable();

    let mut prev = 1;
    for n in arr.into_iter().skip(1) {
        if n - prev > 1 {
            prev += 1;
        } else {
            prev = n;
        }
    }

    prev
}

fn main() {
    let ret = maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        maximum_element_after_decrementing_and_rearranging(vec![73,98,9]),
        3
    );
    assert_eq!(
        maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
        2
    );
    assert_eq!(
        maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
        3
    );
    assert_eq!(
        maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
        5
    );
}
