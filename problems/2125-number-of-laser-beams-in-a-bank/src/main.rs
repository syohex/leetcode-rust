fn number_of_beams(bank: Vec<String>) -> i32 {
    let beams: Vec<i32> = bank
        .iter()
        .filter(|s| !s.chars().all(|c| c == '0'))
        .map(|s| s.chars().filter(|c| *c == '1').count() as i32)
        .collect();

    let mut ret = 0;
    for i in 1..beams.len() {
        ret += beams[i] * beams[i - 1];
    }

    ret
}

fn main() {
    let bank = vec![
        "011001".to_string(),
        "000000".to_string(),
        "010100".to_string(),
        "001000".to_string(),
    ];
    let ret = number_of_beams(bank);
    println!("ret={ret}");
}

#[test]
fn test_number_of_beams() {
    {
        let bank = vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string(),
        ];
        assert_eq!(number_of_beams(bank), 8);
    }
    {
        let bank = vec!["000".to_string(), "111".to_string(), "000".to_string()];
        assert_eq!(number_of_beams(bank), 0);
    }
}
