fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    fn count_bits(n: i32) -> i32 {
        let mut n = n;
        n = (n & 0x55555555) + ((n >> 1) & 0x55555555);
        n = (n & 0x33333333) + ((n >> 2) & 0x33333333);
        n = (n & 0x0f0f0f0f) + ((n >> 4) & 0x0f0f0f0f);
        n = (n & 0x00ff00ff) + ((n >> 8) & 0x00ff00ff);
        n = (n & 0x0000ffff) + ((n >> 16) & 0x0000ffff);
        n
    }

    let mut arr = arr;
    arr.sort_unstable_by(|&a, &b| match count_bits(a).cmp(&count_bits(b)) {
        std::cmp::Ordering::Equal => a.cmp(&b),
        n => n,
    });

    arr
}

fn main() {
    let ret = sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(
        sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        [0, 1, 2, 4, 8, 3, 5, 6, 7]
    );
    assert_eq!(
        sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
        [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );
}
