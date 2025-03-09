fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let orig_len = colors.len();
    let k = k as usize;
    let mut colors = colors;
    for i in 0..k {
        colors.push(colors[i]);
    }

    let mut start = 0;
    let mut end = 1;

    let mut ret = 0;
    while start < orig_len {
        if colors[end - 1] == colors[end] {
            start = end;
            end += 1;
            continue;
        }

        end += 1;
        if end - start < k {
            continue;
        }

        ret += 1;
        start += 1;
    }

    ret
}

fn main() {
    let colors = vec![0, 1, 0, 1, 0];
    let k = 3;
    let ret = number_of_alternating_groups(colors, k);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let colors = vec![0, 1, 0, 1, 0];
        let k = 3;
        let ret = number_of_alternating_groups(colors, k);
        assert_eq!(ret, 3);
    }
    {
        let colors = vec![0, 1, 0, 0, 1, 0, 1];
        let k = 6;
        let ret = number_of_alternating_groups(colors, k);
        assert_eq!(ret, 2);
    }
    {
        let colors = vec![1, 1, 0, 1];
        let k = 4;
        let ret = number_of_alternating_groups(colors, k);
        assert_eq!(ret, 0);
    }
}
