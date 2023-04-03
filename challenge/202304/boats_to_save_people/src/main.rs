fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort_unstable();

    let mut left = 0i32;
    let mut right = people.len() as i32 - 1;

    let mut ret = 0;
    while left <= right {
        ret += 1;

        if people[left as usize] + people[right as usize] <= limit {
            left += 1;
        }

        right -= 1;
    }

    ret
}

fn main() {
    let people = vec![1, 2];
    let ret = num_rescue_boats(people, 3);
    println!("ret={ret}");
}

#[test]
fn test_num_rescue_boats() {
    {
        let people = vec![1, 2];
        let ret = num_rescue_boats(people, 3);
        assert_eq!(ret, 1);
    }
    {
        let people = vec![3, 2, 2, 1];
        let ret = num_rescue_boats(people, 3);
        assert_eq!(ret, 3);
    }
    {
        let people = vec![3, 5, 3, 4];
        let ret = num_rescue_boats(people, 5);
        assert_eq!(ret, 4);
    }
}
