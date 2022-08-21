fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let k = k as usize;
    let cs: Vec<char> = blocks.chars().collect();

    let mut sum = 0;
    for i in 0..k {
        if cs[i] == 'W' {
            sum += 1;
        }
    }

    let mut ret = sum;
    for i in k..cs.len() {
        if cs[i - k] == 'W' {
            sum -= 1;
        }
        if cs[i] == 'W' {
            sum += 1;
        }

        ret = std::cmp::min(ret, sum);
    }

    ret
}

fn main() {
    let blocks = "WBBWWBBWBW".to_string();
    let ret = minimum_recolors(blocks, 7);
    println!("ret={ret}");
}

#[test]
fn test_minimum_recolors() {
    {
        let blocks = "WBBWWBBWBW".to_string();
        let ret = minimum_recolors(blocks, 7);
        assert_eq!(ret, 3);
    }
    {
        let blocks = "WBWBBBW".to_string();
        let ret = minimum_recolors(blocks, 2);
        assert_eq!(ret, 0);
    }
}
