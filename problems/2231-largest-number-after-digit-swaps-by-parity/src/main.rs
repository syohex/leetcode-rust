fn largest_integer(num: i32) -> i32 {
    let f = |n: i32| -> Vec<i32> {
        let mut ret = vec![];
        let mut n = n;

        while n > 0 {
            ret.push(n % 10);
            n /= 10;
        }

        ret.reverse();
        ret
    };

    let mut v = f(num);
    let len = v.len();

    for i in 0..len - 1 {
        let mut max = v[i];
        let mut max_index = i;
        let is_odd = v[i] % 2 == 1;
        for j in i + 1..len {
            if (is_odd && v[j] % 2 == 0) || (!is_odd && v[j] % 2 != 0) {
                continue;
            }

            if v[j] > max {
                max = v[j];
                max_index = j;
            }
        }

        v.swap(i, max_index);
    }

    v.into_iter().fold(0, |acc, n| acc * 10 + n)
}

fn main() {
    let ret = largest_integer(65875);
    println!("ret={ret}");
}

#[test]
fn test_largest_integer() {
    assert_eq!(largest_integer(1234), 3412);
    assert_eq!(largest_integer(65875), 87655);
    assert_eq!(largest_integer(251), 251);
}
