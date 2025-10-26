struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let pos1 = (account1 - 1) as usize;
        let pos2 = (account2 - 1) as usize;
        let len = self.balance.len();
        if pos1 >= len || pos2 >= len {
            false
        } else {
            if self.balance[pos1] < money {
                false
            } else {
                self.balance[pos2] += money;
                self.balance[pos1] -= money;
                true
            }
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let pos = (account - 1) as usize;
        if pos >= self.balance.len() {
            false
        } else {
            self.balance[pos] += money;
            true
        }
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let pos = (account - 1) as usize;
        if pos >= self.balance.len() {
            false
        } else {
            if self.balance[pos] < money {
                false
            } else {
                self.balance[pos] -= money;
                true
            }
        }
    }
}

fn main() {
    let mut b = Bank::new(vec![10, 100, 20, 50, 30]);
    assert!(b.withdraw(3, 10));
    assert!(b.transfer(5, 1, 20));
    assert!(b.deposit(5, 20));
    assert!(!b.transfer(3, 4, 15));
    assert!(!b.withdraw(10, 50));
    println!("ret={:?}", b.balance);
}

#[test]
fn test() {
    {
        let mut b = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(b.withdraw(3, 10));
        assert!(b.transfer(5, 1, 20));
        assert!(b.deposit(5, 20));
        assert!(!b.transfer(3, 4, 15));
        assert!(!b.withdraw(10, 50));
    }
}
