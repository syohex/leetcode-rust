fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let len = code.len();
    if k == 0 {
        return vec![0; len];
    }

    let mut ret = vec![];
    if k > 0 {
        for i in 0..len {
            let mut sum = 0;
            for j in 1..=k as usize {
                sum += code[(i + j) % len];
            }

            ret.push(sum);
        }
    } else {
        for i in 0..len {
            let mut sum = 0;
            let k = -k;
            for j in 1..=k as usize {
                sum += code[(len + i - j) % len];
            }
            ret.push(sum);
        }
    }
    ret
}

fn main() {
    let code = vec![5, 7, 1, 4];
    let ret = decrypt(code, 3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let code = vec![5, 7, 1, 4];
        let expected = vec![12, 10, 16, 13];
        let ret = decrypt(code, 3);
        assert_eq!(ret, expected);
    }
    {
        let code = vec![1, 2, 3, 4];
        let expected = vec![0, 0, 0, 0];
        let ret = decrypt(code, 0);
        assert_eq!(ret, expected);
    }
    {
        let code = vec![2, 4, 9, 3];
        let expected = vec![12, 5, 6, 13];
        let ret = decrypt(code, -2);
        assert_eq!(ret, expected);
    }
}
