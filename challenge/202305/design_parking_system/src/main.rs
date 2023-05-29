struct ParkingSystem {
    cars: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        let v = vec![big, medium, small];
        Self { cars: v }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let index = car_type as usize - 1;
        if self.cars[index] == 0 {
            return false;
        }

        self.cars[index] -= 1;
        true
    }
}

fn main() {
    let mut p = ParkingSystem::new(1, 1, 0);
    println!("ret={}", p.add_car(1));
    println!("ret={}", p.add_car(2));
    println!("ret={}", p.add_car(3));
    println!("ret={}", p.add_car(1));
}

#[test]
fn test_parking_system() {
    {
        let mut p = ParkingSystem::new(1, 1, 0);
        assert!(p.add_car(1));
        assert!(p.add_car(2));
        assert!(!p.add_car(3));
        assert!(!p.add_car(1));
    }
}
