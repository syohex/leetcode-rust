fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut ret = String::new();
    let mut j = 0;
    for (i, c) in s.chars().enumerate() {
        if j < spaces.len() && i == spaces[j] as usize {
            ret.push(' ');
            j += 1;
        }

        ret.push(c);
    }

    ret
}

fn main() {
    let s = "icodeinpython".to_string();
    let spaces = vec![1, 5, 7, 9];
    let ret = add_spaces(s, spaces);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "LeetcodeHelpsMeLearn".to_string();
        let spaces = vec![8, 13, 15];
        let expected = "Leetcode Helps Me Learn";
        let ret = add_spaces(s, spaces);
        assert_eq!(ret, expected);
    }
    {
        let s = "icodeinpython".to_string();
        let spaces = vec![1, 5, 7, 9];
        let expected = "i code in py thon";
        let ret = add_spaces(s, spaces);
        assert_eq!(ret, expected);
    }
    {
        let s = "spacing".to_string();
        let spaces = vec![0, 1, 2, 3, 4, 5, 6];
        let expected = " s p a c i n g";
        let ret = add_spaces(s, spaces);
        assert_eq!(ret, expected);
    }
}
