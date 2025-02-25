fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    let modulo = 1_000_000_007;

    let mut acc = vec![0; arr.len() + 1];
    for (i, a) in arr.into_iter().enumerate() {
        acc[i + 1] = acc[i] + a;
    }

    let mut ret = 0;
    for i in 1..acc.len() {
        for j in 0..i {
            let diff = acc[i] - acc[j];
            if diff % 2 != 0 {
                ret = (ret + 1) % modulo;
            }
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
