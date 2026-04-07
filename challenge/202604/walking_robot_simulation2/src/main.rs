struct Robot {
    positions: Vec<(usize, usize)>,
    directions: Vec<usize>,
    index: usize,
    is_started: bool,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let mut positions = vec![];
        let mut directions = vec![];
        let (width, height) = (width as usize, height as usize);

        for i in 0..width {
            positions.push((i, 0));
            directions.push(0);
        }
        for i in 1..height {
            positions.push((width - 1, i));
            directions.push(1);
        }
        for i in (0..(width - 1)).rev() {
            positions.push((i, height - 1));
            directions.push(2);
        }
        for i in (1..(height - 1)).rev() {
            positions.push((0, i));
            directions.push(3);
        }
        directions[0] = 3;

        Self {
            positions,
            directions,
            index: 0,
            is_started: false,
        }
    }

    fn step(&mut self, num: i32) {
        self.is_started = true;
        self.index = (self.index + num as usize) % self.positions.len();
    }

    fn get_pos(&self) -> Vec<i32> {
        let v = self.positions[self.index];
        vec![v.0 as i32, v.1 as i32]
    }

    fn get_dir(&self) -> String {
        let dirs = ["East", "North", "West", "South"];
        if self.is_started {
            dirs[self.directions[self.index]].to_string()
        } else {
            dirs[0].to_string()
        }
    }
}

fn main() {
    let mut r = Robot::new(6, 3);
    r.step(2);
    r.step(2);
    println!("pos={:?}", r.get_pos());
    println!("dir={}", r.get_dir());
    r.step(2);
    r.step(1);
    r.step(4);
    println!("pos={:?}", r.get_pos());
    println!("dir={}", r.get_dir());
}

#[test]
fn test() {
    {
        let mut r = Robot::new(8, 2);
        r.step(17);
        assert_eq!(r.get_pos(), vec![1, 0]);
        assert_eq!(r.get_dir(), "East");
        r.step(21);
        assert_eq!(r.get_pos(), vec![6, 0]);
        assert_eq!(r.get_dir(), "East");
        r.step(22);
        r.step(34);
        assert_eq!(r.get_pos(), vec![1, 1]);
        assert_eq!(r.get_dir(), "West");
        r.step(1);
        r.step(46);
        r.step(35);
        assert_eq!(r.get_pos(), vec![0, 0]);
        assert_eq!(r.get_dir(), "South");
        r.step(44);
        r.step(14);
        r.step(31);
        assert_eq!(r.get_pos(), vec![6, 1]);
        assert_eq!(r.get_dir(), "West");
    }
    {
        let mut r = Robot::new(6, 3);
        r.step(2);
        r.step(2);
        assert_eq!(r.get_pos(), vec![4, 0]);
        assert_eq!(r.get_dir(), "East");
        r.step(2);
        r.step(1);
        r.step(4);
        assert_eq!(r.get_pos(), vec![1, 2]);
        assert_eq!(r.get_dir(), "West");
    }
}
