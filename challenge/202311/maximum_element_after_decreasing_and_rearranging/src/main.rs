fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
    let mut arr = arr;
    arr.sort_unstable();

    let mut prev = 1;
    for n in arr.into_iter().skip(1) {
        if n - prev > 1 {
            prev = prev + 1;
        } else {
            prev = n;
        }
    }

    prev
}
fn main() {
    let arr = vec![100, 1, 1000];
    let ret = maximum_element_after_decrementing_and_rearranging(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![2, 2, 1, 2, 1];
        let ret = maximum_element_after_decrementing_and_rearranging(arr);
        assert_eq!(ret, 2);
    }
    {
        let arr = vec![100, 1, 1000];
        let ret = maximum_element_after_decrementing_and_rearranging(arr);
        assert_eq!(ret, 3);
    }
    {
        let arr = vec![1, 2, 3, 4, 5];
        let ret = maximum_element_after_decrementing_and_rearranging(arr);
        assert_eq!(ret, 5);
    }
}
