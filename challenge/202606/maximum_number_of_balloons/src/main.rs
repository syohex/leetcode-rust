fn max_number_of_balloons(text: String) -> i32 {
    let mut freq = text.bytes().fold([0; 26], |mut acc, b| {
        let idx = (b - b'a') as usize;
        acc[idx] += 1;
        acc
    });

    let mut ret = 0;
    let l_idx = (b'l' - b'a') as usize;
    let n_idx = (b'n' - b'a') as usize;
    let o_idx = (b'o' - b'a') as usize;
    while freq[0] >= 1 && freq[1] >= 1 && freq[l_idx] >= 2 && freq[n_idx] >= 1 && freq[o_idx] >= 2 {
        freq[0] -= 1;
        freq[1] -= 1;
        freq[l_idx] -= 2;
        freq[n_idx] -= 1;
        freq[o_idx] -= 2;
        ret += 1;
    }

    ret
}

fn main() {
    let ret = max_number_of_balloons("nlaebolko".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1);
    assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    assert_eq!(max_number_of_balloons("leetcode".to_string()), 0);
}
