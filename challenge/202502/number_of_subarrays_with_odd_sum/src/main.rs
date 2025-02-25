fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let modulo = 1_000_000_007;

    let mut ret = 0;
    let mut odds = 0;
    let mut evens = 1;
    let mut acc = 0;
    for a in arr {
        acc += a;
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
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let ret = num_of_subarrays(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![1, 3, 5];
        let ret = num_of_subarrays(arr);
        assert_eq!(ret, 4);
    }
    {
        let arr = vec![2, 4, 6];
        let ret = num_of_subarrays(arr);
        assert_eq!(ret, 0);
    }
    {
        let arr = vec![1, 2, 3, 4, 5, 6, 7];
        let ret = num_of_subarrays(arr);
        assert_eq!(ret, 16);
    }
}
