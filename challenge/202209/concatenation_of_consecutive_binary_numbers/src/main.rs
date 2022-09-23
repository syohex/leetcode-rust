fn concatenated_binary(n: i32) -> i32 {
    let mut bits = vec![];
    for i in 1..=n {
        let mut tmp = vec![0; 32];
        let mut m = i;
        let mut j = 0;

        while m > 0 {
            tmp[j] = m % 2;
            j += 1;
            m /= 2;
        }

        for k in (0..j).rev() {
            bits.push(tmp[k]);
        }
    }

    let modulo = 1_000_000_007i64;
    let mut sum = 0i64;
    for b in bits {
        sum = (sum * 2 + b as i64) % modulo;
    }

    sum as i32
}

fn main() {
    let n = 12;
    let ret = concatenated_binary(n);
    println!("ret={ret}");
}

#[test]
fn test_concatenated_binary() {
    {
        let n = 3;
        let ret = concatenated_binary(n);
        assert_eq!(ret, 27);
    }
    {
        let n = 12;
        let ret = concatenated_binary(n);
        assert_eq!(ret, 505379714);
    }
}
