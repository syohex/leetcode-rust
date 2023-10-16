fn get_row(row_index: i32) -> Vec<i32> {
    let mut prev = vec![1];
    for i in 1..=(row_index as usize) {
        let mut tmp = vec![];
        for j in 0..=i {
            if j == 0 {
                tmp.push(prev[0]);
            } else if j == i {
                tmp.push(prev[j - 1]);
            } else {
                tmp.push(prev[j - 1] + prev[j]);
            }
        }

        prev = tmp;
    }

    prev
}

fn main() {
    let ret = get_row(3);
    println!("ret={ret:?}");
}

#[test]
fn test_get_row() {
    {
        let expected = vec![1,3,3,1];
        let ret = get_row(3);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![1];
        let ret = get_row(0);
        assert_eq!(ret, expected);
    }
    {
        let expected = vec![1, 1];
        let ret = get_row(1);
        assert_eq!(ret, expected);
    }
}
