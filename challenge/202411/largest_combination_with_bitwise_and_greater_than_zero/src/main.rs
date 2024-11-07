fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut bits = vec![0; 31];

    for i in 0..31 {
        for &num in &candidates {
            if num & (1 << i) != 0 {
                bits[i] += 1;
            }
        }
    }

    bits.into_iter().max().unwrap()
}

fn main() {
    let candidates = vec![16, 17, 71, 62, 12, 24, 14];
    let ret = largest_combination(candidates);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let candidates = vec![16, 17, 71, 62, 12, 24, 14];
        let ret = largest_combination(candidates);
        assert_eq!(ret, 4);
    }
    {
        let candidates = vec![8, 8];
        let ret = largest_combination(candidates);
        assert_eq!(ret, 2);
    }
    {
        let candidates = vec![
            84, 40, 66, 44, 91, 90, 1, 14, 73, 51, 47, 35, 18, 46, 18, 65, 55, 18, 16, 45, 43, 58,
            90, 92, 91, 43, 44, 76, 85, 72, 24, 89, 60, 94, 81, 90, 86, 79, 84, 41, 41, 28, 44,
        ];
        let ret = largest_combination(candidates);
        assert_eq!(ret, 28);
    }
}
