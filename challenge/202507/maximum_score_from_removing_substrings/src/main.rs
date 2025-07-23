fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    fn f(cs: &[char], c1: char, c2: char, score: i32) -> (Vec<char>, i32) {
        let mut ret = vec![];
        let mut point = 0;
        for &c in cs {
            if c == c2 {
                if !ret.is_empty() && *ret.last().unwrap() == c1 {
                    ret.pop();
                    point += score;
                    continue;
                }
            }

            ret.push(c);
        }

        (ret, point)
    }

    let cs: Vec<_> = s.chars().collect();
    let mut ret = 0;
    if x > y {
        let (cs, point1) = f(&cs, 'a', 'b', x);
        let (_, point2) = f(&cs, 'b', 'a', y);
        ret += point1 + point2;
    } else {
        let (cs, point1) = f(&cs, 'b', 'a', y);
        let (_, point2) = f(&cs, 'a', 'b', x);
        ret += point1 + point2;
    }

    ret
}

fn main() {
    let s = "cdbcbbaaabab".to_string();
    let ret = maximum_gain(s, 4, 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "cdbcbbaaabab".to_string();
        let ret = maximum_gain(s, 4, 5);
        assert_eq!(ret, 19);
    }
    {
        let s = "aabbaaxybbaabb".to_string();
        let ret = maximum_gain(s, 5, 4);
        assert_eq!(ret, 20);
    }
}
