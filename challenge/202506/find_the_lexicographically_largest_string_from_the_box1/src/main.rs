fn answer_string(word: String, num_friends: i32) -> String {
    if num_friends == 1 {
        return word;
    }

    let num_friends = num_friends as usize;
    let len = word.len();

    let mut ret = "";
    for i in 0..len {
        let last_pos = std::cmp::min(len, i + len - num_friends + 1);
        let substr = &word[i..last_pos];
        ret = std::cmp::max(ret, substr);
    }

    ret.to_string()
}

fn main() {
    let word = "dbca".to_string();
    let num_friends = 2;
    let ret = answer_string(word, num_friends);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let word = "gh".to_string();
        let num_friends = 1;
        let ret = answer_string(word, num_friends);
        assert_eq!(ret, "gh");
    }
    {
        let word = "dbca".to_string();
        let num_friends = 2;
        let ret = answer_string(word, num_friends);
        assert_eq!(ret, "dbc");
    }
    {
        let word = "gggg".to_string();
        let num_friends = 4;
        let ret = answer_string(word, num_friends);
        assert_eq!(ret, "g");
    }
}
