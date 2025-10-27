fn number_of_beams(bank: Vec<String>) -> i32 {
    let devices: Vec<_> = bank
        .into_iter()
        .map(|s| s.chars().filter(|&c| c == '1').count() as i32)
        .filter(|&n| n != 0)
        .collect();

    if devices.len() == 0 {
        0
    } else {
        let mut ret = 0;
        for i in 0..(devices.len() - 1) {
            ret += devices[i] * devices[i + 1];
        }

        ret
    }
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
fn test() {
    {
        let bank = vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string(),
        ];
        let ret = number_of_beams(bank);
        assert_eq!(ret, 8);
    }
    {
        let bank = vec!["000".to_string(), "111".to_string(), "000".to_string()];
        let ret = number_of_beams(bank);
        assert_eq!(ret, 0);
    }
    {
        let bank = vec!["0".to_string()];
        let ret = number_of_beams(bank);
        assert_eq!(ret, 0);
    }
}
