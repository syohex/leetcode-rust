fn max_consective_answers(answer_key: String, k: i32) -> i32 {
    let len = answer_key.len();
    let mut left = 0;
    let mut right = 0;
    let mut trues = 0;
    let mut falses = 0;

    let cs: Vec<char> = answer_key.chars().collect();
    let mut ret = 0;
    while right < len {
        if cs[right] == 'T' {
            trues += 1;
        } else {
            falses += 1;
        }

        while trues > k && falses > k && left <= right && right < len {
            if cs[left] == 'T' {
                trues -= 1;
            } else {
                falses -= 1;
            }

            left += 1;
        }

        ret = std::cmp::max(ret, right - left + 1);
        right += 1;
    }

    ret as i32
}

fn main() {
    let answer_key = "TTFF".to_string();
    let k = 2;
    let ret = max_consective_answers(answer_key, k);
    println!("ret={ret}");
}

#[test]
fn test_max_consecutive_answers() {
    {
        let answer_key = "TTFF".to_string();
        let k = 2;
        let ret = max_consective_answers(answer_key, k);
        assert_eq!(ret, 4);
    }
    {
        let answer_key = "TFFT".to_string();
        let k = 1;
        let ret = max_consective_answers(answer_key, k);
        assert_eq!(ret, 3);
    }
    {
        let answer_key = "TTFTTFTT".to_string();
        let k = 1;
        let ret = max_consective_answers(answer_key, k);
        assert_eq!(ret, 5);
    }
}
