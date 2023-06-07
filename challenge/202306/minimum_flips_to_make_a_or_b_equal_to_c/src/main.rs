fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    fn to_bits(n: i32) -> Vec<bool> {
        let mut v = vec![false; 32];
        let mut mask = 1;

        for i in 0..32 {
            if n & mask != 0 {
                v[i] = true;
            }

            mask <<= 1;
        }

        v
    }

    let av = to_bits(a);
    let bv = to_bits(b);
    let cv = to_bits(c);

    let mut ret = 0;
    for i in 0..32 {
        match cv[i] {
            true => match (av[i], bv[i]) {
                (false, false) => ret += 1,
                _ => (),
            },
            false => match (av[i], bv[i]) {
                (true, false) | (false, true) => ret += 1,
                (true, true) => ret += 2,
                _ => (),
            },
        }
    }

    ret
}

fn main() {
    let ret = min_flips(2, 6, 5);
    println!("ret={ret}");
}

#[test]
fn test_min_flips() {
    {
        let ret = min_flips(2, 6, 5);
        assert_eq!(ret, 3);
    }
    {
        let ret = min_flips(4, 2, 7);
        assert_eq!(ret, 1);
    }
    {
        let ret = min_flips(1, 2, 3);
        assert_eq!(ret, 0);
    }
}
