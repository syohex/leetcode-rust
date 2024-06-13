fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;

    seats.sort_unstable();
    students.sort_unstable();

    let mut ret = 0;
    for i in 0..seats.len() {
        ret += (seats[i] - students[i]).abs();
    }

    ret
}

fn main() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];
    let ret = min_moves_to_seat(seats, students);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        let ret = min_moves_to_seat(seats, students);
        assert_eq!(ret, 4);
    }
    {
        let seats = vec![4, 1, 5, 9];
        let students = vec![1, 3, 2, 6];
        let ret = min_moves_to_seat(seats, students);
        assert_eq!(ret, 7);
    }
}
