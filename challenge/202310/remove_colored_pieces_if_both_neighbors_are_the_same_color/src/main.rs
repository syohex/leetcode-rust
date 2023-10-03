fn winner_of_game(colors: String) -> bool {
    let v: Vec<char> = colors.chars().collect();
    let len = v.len();

    let (alices, bobs) = (1..(len - 1)).into_iter().fold((0, 0), |(a, b), i| {
        if v[i - 1] == v[i] && v[i] == v[i + 1] {
            if v[i] == 'A' {
                (a + 1, b)
            } else {
                (a, b + 1)
            }
        } else {
            (a, b)
        }
    });

    alices > bobs
}

fn main() {
    let colors = "AAABABB".to_string();
    let ret = winner_of_game(colors);
    println!("ret={ret}");
}

#[test]
fn test_winner_of_game() {
    {
        let colors = "AAABABB".to_string();
        let ret = winner_of_game(colors);
        assert!(ret);
    }
    {
        let colors = "ABBBBBBBAA".to_string();
        let ret = winner_of_game(colors);
        assert!(!ret);
    }
    {
        let colors = "AA".to_string();
        let ret = winner_of_game(colors);
        assert!(!ret);
    }
}
