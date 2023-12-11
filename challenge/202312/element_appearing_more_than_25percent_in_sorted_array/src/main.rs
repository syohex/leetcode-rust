fn find_special_integer(arr: Vec<i32>) -> i32 {
    let quarter_len = arr.len() / 4;
    let limit = arr.len() - quarter_len;

    for i in 0..limit {
        if arr[i] == arr[i + quarter_len] {
            return arr[i];
        }
    }

    panic!("never reach here")
}

fn main() {
    let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
    let ret = find_special_integer(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
        let ret = find_special_integer(arr);
        assert_eq!(ret, 6);
    }
    {
        let arr = vec![1, 1];
        let ret = find_special_integer(arr);
        assert_eq!(ret, 1);
    }
}
