fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut box_types = box_types;
    box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));

    let mut ret = 0;
    let mut truck_size = truck_size;
    for b in box_types {
        let num = std::cmp::min(truck_size, b[0]);
        ret += num * b[1];

        truck_size -= num;
        if truck_size <= 0 {
            break;
        }
    }

    ret
}

fn main() {
    let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
    let ret = maximum_units(box_types, 4);
    println!("ret={ret}");
}

#[test]
fn test_maximum_units() {
    {
        let box_types = vec![vec![1, 3], vec![2, 2], vec![3, 1]];
        let ret = maximum_units(box_types, 4);
        assert_eq!(ret, 8);
    }
    {
        let box_types = vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]];
        let ret = maximum_units(box_types, 10);
        assert_eq!(ret, 91);
    }
}
