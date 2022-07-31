struct NumArray {
    accs: Vec<i32>,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut accs = vec![0];
        let mut sum = 0;
        for num in &nums {
            accs.push(sum + num);
            sum += num;
        }

        Self { accs, nums }
    }

    fn update(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let diff = self.nums[index] - val;

        for i in (index + 1)..self.accs.len() {
            self.accs[i] -= diff;
        }

        self.nums[index] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.accs[(right + 1) as usize] - self.accs[left as usize]
    }
}

fn main() {
    let mut na = NumArray::new(vec![1, 3, 5]);
    na.update(1, 2);
    println!("ret={}", na.sum_range(0, 2));
}

#[test]
fn test_range_sum_query() {
    {
        let mut na = NumArray::new(vec![1, 3, 5]);
        assert_eq!(na.sum_range(0, 2), 9);
        na.update(1, 2);
        assert_eq!(na.sum_range(0, 2), 8);
    }
    {
        let mut na = NumArray::new(vec![7, 2, 7, 2, 0]);
        na.update(4, 6);
        na.update(0, 2);
        na.update(0, 9);
        assert_eq!(na.sum_range(4, 4), 6);
        na.update(3, 8);
        assert_eq!(na.sum_range(0, 4), 32);
        na.update(4, 1);
        assert_eq!(na.sum_range(0, 3), 26);
        assert_eq!(na.sum_range(0, 4), 27);
    }
    {
        let mut na = NumArray::new(vec![-1]);
        assert_eq!(na.sum_range(0, 0), -1);
        na.update(0, 1);
        assert_eq!(na.sum_range(0, 0), 1);
    }
}
