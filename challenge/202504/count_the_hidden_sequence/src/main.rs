fn number_of_array(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut v = 0;
    let mut min = 0;
    let mut max = 0;

    for d in differences {
        v += d;
        min = std::cmp::min(min, v);
        max = std::cmp::max(max, v);

        if max - min > upper - lower {
            return 0;
        }
    }

    upper - lower + 1 - (max - min)
}

fn main() {
    let differences = vec![3, -4, 5, 1, -2];
    let ret = number_of_array(differences, -4, 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let differences = vec![1, -3, 4];
        let ret = number_of_array(differences, 1, 6);
        assert_eq!(ret, 2);
    }
    {
        let differences = vec![3, -4, 5, 1, -2];
        let ret = number_of_array(differences, -4, 5);
        assert_eq!(ret, 4);
    }
}
