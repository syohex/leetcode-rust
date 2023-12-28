fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    for i in 1..(mountain.len() - 1) {
        if mountain[i - 1] < mountain[i] && mountain[i] > mountain[i + 1] {
            ret.push(i as i32);
        }
    }

    ret
}

fn main() {
    let mountain = vec![1, 4, 3, 8, 5];
    let ret = find_peaks(mountain);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let mountain = vec![2, 4, 4];
        let expected = vec![];
        let ret = find_peaks(mountain);
        assert_eq!(ret, expected);
    }
    {
        let mountain = vec![1, 4, 3, 8, 5];
        let expected = vec![1, 3];
        let ret = find_peaks(mountain);
        assert_eq!(ret, expected);
    }
}
