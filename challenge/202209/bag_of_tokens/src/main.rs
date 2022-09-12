fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
    let mut power = power;
    let mut tokens = tokens;
    tokens.sort_unstable();

    let mut left = 0i32;
    let mut right = (tokens.len() - 1) as i32;

    let mut ret = 0;
    let mut score = 0;
    while left <= right {
        if power >= tokens[left as usize] {
            score += 1;
            power -= tokens[left as usize];
            left += 1;

            ret = std::cmp::max(ret, score);
        } else if score >= 1 {
            score -= 1;
            power += tokens[right as usize];
            right -= 1;
        } else {
            break;
        }
    }

    ret
}

fn main() {
    let tokens = vec![100, 200];
    let ret = bag_of_tokens_score(tokens, 150);
    println!("ret={ret}");
}

#[test]
fn test_bag_of_tokens_score() {
    {
        let tokens = vec![100];
        let ret = bag_of_tokens_score(tokens, 50);
        assert_eq!(ret, 0);
    }
    {
        let tokens = vec![100, 200];
        let ret = bag_of_tokens_score(tokens, 150);
        assert_eq!(ret, 1);
    }
    {
        let tokens = vec![100, 200, 300, 400];
        let ret = bag_of_tokens_score(tokens, 200);
        assert_eq!(ret, 2);
    }
    {
        let tokens = vec![71, 55, 82];
        let ret = bag_of_tokens_score(tokens, 54);
        assert_eq!(ret, 0);
    }
    {
        let tokens = vec![26];
        let ret = bag_of_tokens_score(tokens, 51);
        assert_eq!(ret, 1);
    }
}
