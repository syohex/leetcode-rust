fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let cs: Vec<_> = blocks.chars().collect();
    let k = k as usize;
    let mut whites = 0;

    for &c in cs.iter().take(k) {
        if c == 'W' {
            whites += 1;
        }
    }

    let mut ret = whites;
    for i in k..cs.len() {
        if cs[i - k] == 'W' {
            whites -= 1;
        }
        if cs[i] == 'W' {
            whites += 1;
        }

        ret = std::cmp::min(ret, whites);
    }

    ret
}

fn main() {
    let blocks = "WBBWWBBWBW".to_string();
    let ret = minimum_recolors(blocks, 7);
    println!("ret={ret}");
}

#[test]
fn test() {
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
