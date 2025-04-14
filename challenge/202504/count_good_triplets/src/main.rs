fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len = arr.len();
    let mut ret = 0;
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            if (arr[i] - arr[j]).abs() > a {
                continue;
            }

            for k in (j + 1)..len {
                if (arr[j] - arr[k]).abs() > b || (arr[i] - arr[k]).abs() > c {
                    continue;
                }

                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let arr = vec![3, 0, 1, 1, 9, 7];
    let ret = count_good_triplets(arr, 7, 2, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![7, 3, 7, 3, 12, 1, 12, 2, 3];
        let ret = count_good_triplets(arr, 5, 8, 1);
        assert_eq!(ret, 12);
    }
    {
        let arr = vec![3, 0, 1, 1, 9, 7];
        let ret = count_good_triplets(arr, 7, 2, 3);
        assert_eq!(ret, 4);
    }
    {
        let arr = vec![1, 1, 2, 2, 3];
        let ret = count_good_triplets(arr, 0, 0, 1);
        assert_eq!(ret, 0);
    }
}
