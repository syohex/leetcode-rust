fn longest_palindrome(s: String) -> String {
    let cs: Vec<char> = s.chars().collect();
    let len = s.len();
    let mut dp = vec![vec![false; len]; len];

    for i in 0..len {
        dp[i][i] = true;
    }

    let mut start = 0;
    let mut end = 0;
    for i in 0..len - 1 {
        if cs[i] == cs[i + 1] {
            dp[i][i + 1] = true;
            start = i;
            end = i + 1;
        }
    }

    for i in 2..len {
        let limit = len - i;
        for j in 0..limit {
            if cs[j] == cs[j + i] && dp[j + 1][j + i - 1] {
                dp[j][j + i] = true;
                start = j;
                end = j + i;
            }
        }
    }

    s[start..=end].to_string()
}

fn main() {
    let s = "babad".to_string();
    let ret = longest_palindrome(s);
    println!("ret={ret}");
}

#[test]
fn test_longest_palindrome() {
    {
        let s = "babad".to_string();
        let ret = longest_palindrome(s);
        assert_eq!(&ret, "aba");
    }
    {
        let s = "cbbd".to_string();
        let ret = longest_palindrome(s);
        assert_eq!(&ret, "bb");
    }
    {
        let s = "cwziydanrqvsdtvnnqgjnbrvvwxwqojeqgxhwxdoktjktulemwpbeqscbbtbfvkxsrjetfdrovcrdwzfmnnihtgxybuairswfewvpuscocqifuwylhssldpjrawqdrbvkykpaggspbfrulcktpbofchzikhzxhpocgvdbwpewpywsgqbczmamprklaoovcfecwchhmsaqkhvuvvzjblmgvqpqtnlipgqsanvovylpmxlmxvymppdykphhaamtxjnnlsqfwjwhyywgurteaummwhvavxbcpgrfffxrowluqmqjaugryxdmwvyokdcfcvcytxpixbvwrdgzctejdoaavgtezexmvxgrkpnayvfarkyoruofqmpnsqdzojxqrjsnfwsbzjmaoigytygukqlrcqaxazvmytgfghdczvzphfdbnxtklaiqqsotavdmhiaermluafheowcobjqmrkmlzyas".to_string();
        let ret = longest_palindrome(s);
        assert_eq!(&ret, "gytyg");
    }
}
