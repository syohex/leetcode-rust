fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<(i32, u32)> = arr.into_iter().map(|n| (n, n.count_ones())).collect();
    arr.sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    arr.into_iter().map(|a| a.0).collect()
}

fn main() {
    let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let ret = sort_by_bits(arr);
    println!("ret={ret:?}");
}

#[test]
fn test_sort_by_bits() {
    {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![0, 1, 2, 4, 8, 3, 5, 6, 7];
        let ret = sort_by_bits(arr);
        assert_eq!(ret, expected);
    }
    {
        let arr = vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        let expected = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        let ret = sort_by_bits(arr);
        assert_eq!(ret, expected);
    }
}
