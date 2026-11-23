struct ProductOfNumbers {
    nums: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { nums: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums = vec![1];
        } else {
            self.nums.push(self.nums[self.nums.len() - 1] * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let len = self.nums.len();
        if k >= len {
            return 0;
        }

        self.nums[len - 1] / self.nums[len - 1 - k]
    }
}

fn main() {
    let mut p = ProductOfNumbers::new();
    p.add(3);
    p.add(0);
    p.add(2);
    p.add(5);
    p.add(4);
    assert_eq!(p.get_product(2), 20);
    assert_eq!(p.get_product(3), 40);
    assert_eq!(p.get_product(4), 0);
    p.add(8);
    assert_eq!(p.get_product(2), 32);
}
