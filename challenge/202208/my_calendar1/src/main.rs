struct MyCalendar {
    ranges: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Self { ranges: vec![] }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for (s, e) in &self.ranges {
            if end <= *s {
                continue;
            }

            if (start <= *s && end > *s) || start < *e {
                return false;
            }
        }

        self.ranges.push((start, end));
        true
    }
}

fn main() {
    let mut cal = MyCalendar::new();
    let ret1 = cal.book(10, 20);
    let ret2 = cal.book(15, 25);
    let ret3 = cal.book(20, 30);
    println!("ret={ret1}, {ret2}, {ret3}");
}

#[test]
fn test_my_calendar() {
    {
        let mut cal = MyCalendar::new();
        assert!(cal.book(10, 20));
        assert!(!cal.book(15, 25));
        assert!(cal.book(20, 30));
    }
    {
        let mut cal = MyCalendar::new();
        assert!(cal.book(47, 50));
        assert!(cal.book(33, 41));
        assert!(!cal.book(39, 45));
        assert!(!cal.book(33, 42));
        assert!(cal.book(25, 32));
        assert!(!cal.book(26, 35));
        assert!(cal.book(19, 25));
        assert!(cal.book(3, 8));
        assert!(cal.book(8, 13));
        assert!(!cal.book(18, 27));
    }
    {
        let mut cal = MyCalendar::new();
        assert!(cal.book(20, 29));
        assert!(!cal.book(13, 22));
        assert!(cal.book(44, 50));
        assert!(cal.book(1, 7));
        assert!(!cal.book(2, 10));
        assert!(cal.book(14, 20));
        assert!(!cal.book(19, 25));
        assert!(cal.book(36, 42));
        assert!(!cal.book(45, 50));
        assert!(!cal.book(47, 50));
        assert!(!cal.book(39, 45));
        assert!(!cal.book(44, 50));
        assert!(!cal.book(16, 25));
        assert!(!cal.book(45, 50));
        assert!(!cal.book(45, 50));
        assert!(!cal.book(12, 20));
        assert!(!cal.book(21, 29));
        assert!(!cal.book(11, 20));
        assert!(!cal.book(12, 17));
        assert!(!cal.book(34, 40));
        assert!(!cal.book(10, 18));
        assert!(!cal.book(38, 44));
        assert!(!cal.book(23, 32));
        assert!(!cal.book(38, 44));
        assert!(!cal.book(15, 20));
        assert!(!cal.book(27, 33));
        assert!(!cal.book(34, 42));
        assert!(!cal.book(44, 50));
        assert!(!cal.book(35, 40));
        assert!(!cal.book(24, 31));
    }
}
