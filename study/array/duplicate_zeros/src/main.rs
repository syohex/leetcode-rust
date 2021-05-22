fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == 0 {
            let mut j = arr.len() - 1;
            while i < j {
                arr[j] = arr[j - 1];
                j -= 1;
            }

            if i + 1 < arr.len() {
                arr[i + 1] = 0;
            }
            i += 2;
        } else {
            i += 1;
        }
    }
}

fn main() {
    let mut nums = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut nums);
    println!("nums={:?}", nums);
}

#[test]
fn test_duplicate_zeros() {
    {
        let mut nums = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }
    {
        let mut nums = vec![1, 2, 3];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }
    {
        let mut nums = vec![0];
        duplicate_zeros(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
