fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let len = needed_time.len();
    let cs: Vec<char> = colors.chars().collect();

    let mut i = 0;
    let mut ret = 0;
    while i < len {
        let mut j = i;
        let mut total = 0;
        let mut max = 0;
        while j < len && cs[i] == cs[j] {
            total += needed_time[j];
            max = std::cmp::max(max, needed_time[j]);
            j += 1;
        }

        ret += total - max;
        i = j;
    }

    ret
}

fn main() {
    let colors = "abaac".to_string();
    let needed_time = vec![1, 2, 3, 4, 5];
    let ret = min_cost(colors, needed_time);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let colors = "abaac".to_string();
        let needed_time = vec![1, 2, 3, 4, 5];
        let ret = min_cost(colors, needed_time);
        assert_eq!(ret, 3);
    }
    {
        let colors = "abc".to_string();
        let needed_time = vec![1, 2, 3];
        let ret = min_cost(colors, needed_time);
        assert_eq!(ret, 0);
    }
    {
        let colors = "aabaa".to_string();
        let needed_time = vec![1, 2, 3, 4, 1];
        let ret = min_cost(colors, needed_time);
        assert_eq!(ret, 2);
    }
}
