fn count_bits(n: i32) -> Vec<i32> {
    let mut ret = vec![];

    for i in 0..=n {
        let mut i = i;
        i = (i & 0x55555555) + ((i >> 1) & 0x55555555);
        i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
        i = (i & 0x0F0F0F0F) + ((i >> 4) & 0x0F0F0F0F);
        i = (i & 0x00FF00FF) + ((i >> 8) & 0x00FF00FF);
        i = (i & 0x0000FFFF) + ((i >> 16) & 0x0000FFFF);
        ret.push(i);
    }

    ret
}

fn main() {
    let ret = count_bits(16);
    println!("ret={ret:?}");
}

#[test]
fn test_count_bits() {
    {
        let n = 2;
        let expected = vec![0, 1, 1];
        let ret = count_bits(n);
        assert_eq!(ret, expected);
    }
    {
        let n = 5;
        let expected = vec![0, 1, 1, 2, 1, 2];
        let ret = count_bits(n);
        assert_eq!(ret, expected);
    }
}
