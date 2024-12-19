fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut windows = vec![];
    for (i, n) in arr.into_iter().enumerate() {
        let i = i as i32;
        if n >= i {
            windows.push(n);
        } else {
            windows.push(i);
        }
    }

    let mut ret = 0;
    let mut window_right = 0;
    for (i, right) in windows.into_iter().enumerate() {
        let i = i as i32;
        window_right = std::cmp::max(window_right, right);
        if window_right <= i {
            ret += 1;
        }
    }
    ret
}

fn main() {
    let arr = vec![1, 0, 2, 3, 4];
    let ret = max_chunks_to_sorted(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![0, 4, 2, 3, 1];
        let ret = max_chunks_to_sorted(arr);
        assert_eq!(ret, 2);
    }
    {
        let arr = vec![4, 3, 2, 1, 0];
        let ret = max_chunks_to_sorted(arr);
        assert_eq!(ret, 1);
    }
    {
        let arr = vec![1, 0, 2, 3, 4];
        let ret = max_chunks_to_sorted(arr);
        assert_eq!(ret, 4);
    }
}
