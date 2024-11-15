fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let mut right = arr.len() - 1;
    while right > 0 && arr[right - 1] <= arr[right] {
        right -= 1;
    }

    let mut left = 0;
    while left < arr.len() - 1 && arr[left] <= arr[left + 1] {
        left += 1;
    }

    if left == arr.len() - 1 {
        return 0;
    }

    let mut ret = std::cmp::min(arr.len() - left - 1, right) as i32;
    let mut i = 0;
    while i <= left && right < arr.len() {
        if arr[i] <= arr[right] {
            ret = std::cmp::min(ret, right as i32 - i as i32 - 1);
            i += 1;
        } else {
            right += 1;
        }
    }

    ret
}

fn main() {
    let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
    let ret = find_length_of_shortest_subarray(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
        let ret = find_length_of_shortest_subarray(arr);
        assert_eq!(ret, 3);
    }
    {
        let arr = vec![5, 4, 3, 2, 1];
        let ret = find_length_of_shortest_subarray(arr);
        assert_eq!(ret, 4);
    }
    {
        let arr = vec![1, 2, 3];
        let ret = find_length_of_shortest_subarray(arr);
        assert_eq!(ret, 0);
    }
    {
        let arr = vec![2, 2, 2, 1, 1, 1];
        let ret = find_length_of_shortest_subarray(arr);
        assert_eq!(ret, 3);
    }
    {
        let arr = vec![10, 13, 17, 21, 15, 15, 9, 17, 22, 22, 13];
        let ret = find_length_of_shortest_subarray(arr);
        assert_eq!(ret, 7);
    }
}
