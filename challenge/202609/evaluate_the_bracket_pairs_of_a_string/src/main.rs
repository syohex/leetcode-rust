fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    use std::collections::HashMap;

    let mut dict: HashMap<&str, &str> = HashMap::new();
    for k in &knowledge {
        dict.insert(k[0].as_str(), k[1].as_str());
    }

    let mut ret = String::new();
    let cs: Vec<_> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        if cs[i] == '(' {
            i += 1;
            let mut tmp = String::new();
            while cs[i] != ')' {
                tmp.push(cs[i]);
                i += 1;
            }

            if let Some(v) = dict.get(tmp.as_str()) {
                ret.push_str(v);
            } else {
                ret.push('?');
            }

            i += 1;
        } else {
            ret.push(cs[i]);
            i += 1;
        }
    }

    ret
}
fn main() {
    let s = "(name)is(age)yearsold".to_string();
    let knowledge = vec![
        vec!["name".to_string(), "bob".to_string()],
        vec!["age".to_string(), "two".to_string()],
    ];
    let ret = evaluate(s, knowledge);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "(name)is(age)yearsold".to_string();
        let knowledge = vec![
            vec!["name".to_string(), "bob".to_string()],
            vec!["age".to_string(), "two".to_string()],
        ];
        let ret = evaluate(s, knowledge);
        assert_eq!(ret, "bobistwoyearsold");
    }
    {
        let s = "hi(name)".to_string();
        let knowledge = vec![vec!["a".to_string(), "b".to_string()]];
        let ret = evaluate(s, knowledge);
        assert_eq!(ret, "hi?");
    }
    {
        let s = "(a)(a)(a)aaa".to_string();
        let knowledge = vec![vec!["a".to_string(), "yes".to_string()]];
        let ret = evaluate(s, knowledge);
        assert_eq!(ret, "yesyesyesaaa");
    }
}
