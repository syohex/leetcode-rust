fn num_rescure_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort_unstable();

    let mut left = 0i32;
    let mut right = (people.len() - 1) as i32;

    let mut ret = 0;
    while left <= right {
        ret += 1;

        if people[left as usize] + people[right as usize] <= limit {
            left += 1;
            right -= 1;
        } else {
            right -= 1;
        }
    }

    ret
}

fn main() {
    let people = vec![3, 5, 3, 4];
    let ret = num_rescure_boats(people, 5);
    println!("ret={ret}");
}

#[test]
fn test_num_rescure_boats() {
    {
        let people = vec![1, 2];
        assert_eq!(num_rescure_boats(people, 3), 1);
    }
    {
        let people = vec![3, 2, 2, 1];
        assert_eq!(num_rescure_boats(people, 3), 3);
    }
    {
        let people = vec![3, 5, 3, 4];
        assert_eq!(num_rescure_boats(people, 5), 4);
    }
    {
        let people = vec![1];
        assert_eq!(num_rescure_boats(people, 1), 1);
    }
}
