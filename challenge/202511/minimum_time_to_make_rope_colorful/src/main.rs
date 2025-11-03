fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let cs: Vec<_> = colors.chars().collect();

    let mut ret = 0;
    let mut i = 0;
    while i < cs.len() {
        let mut j = i;
        let mut max = 0;
        let mut sum = 0;

        while j < cs.len() && cs[i] == cs[j] {
            sum += needed_time[j];
            max = std::cmp::max(max, needed_time[j]);
            j += 1;
        }

        ret += sum - max;
        i = j;
    }

    ret
}

fn main() {
    let colors = String::from("abaac");
    let needed_time = vec![1, 2, 3, 4, 5];
    let ret = min_cost(colors, needed_time);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let colors = String::from("bbbaaa");
        let needed_time = vec![4, 9, 3, 8, 8, 9];
        let ret = min_cost(colors, needed_time);
        assert_eq!(ret, 23);
    }
    {
        let colors = String::from("abaac");
        let needed_time = vec![1, 2, 3, 4, 5];
        let ret = min_cost(colors, needed_time);
        assert_eq!(ret, 3);
    }
    {
        let colors = String::from("abc");
        let needed_time = vec![1, 2, 3];
        let ret = min_cost(colors, needed_time);
        assert_eq!(ret, 0);
    }
    {
        let colors = String::from("aabaa");
        let needed_time = vec![1, 2, 3, 4, 1];
        let ret = min_cost(colors, needed_time);
        assert_eq!(ret, 2);
    }
}
