pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    v: Vec<i32>,
    pos: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn flatten(nested_list: &Vec<NestedInteger>, acc: &mut Vec<i32>) {
        for n in nested_list {
            match n {
                NestedInteger::Int(m) => acc.push(*m),
                NestedInteger::List(v) => {
                    let mut acc2 = vec![];
                    NestedIterator::flatten(v, &mut acc2);
                    acc.append(&mut acc2);
                }
            }
        }
    }

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut acc = vec![];
        NestedIterator::flatten(&nested_list, &mut acc);

        Self { v: acc, pos: 0 }
    }

    fn next(&mut self) -> i32 {
        let ret = self.v[self.pos];
        self.pos += 1;
        ret
    }

    fn has_next(&self) -> bool {
        self.pos < self.v.len()
    }
}

fn check(ni: &mut NestedIterator) -> Vec<i32> {
    let mut ret = vec![];
    while ni.has_next() {
        ret.push(ni.next());
    }

    ret
}

fn main() {
    use NestedInteger::*;
    let mut ni = NestedIterator::new(vec![
        List(vec![Int(1), Int(1)]),
        Int(2),
        List(vec![Int(1), Int(1)]),
    ]);
    let ret = check(&mut ni);
    println!("ret={:?}", ret);
}

#[test]
fn test_nested_iterator() {
    use NestedInteger::*;

    {
        let mut ni = NestedIterator::new(vec![
            List(vec![Int(1), Int(1)]),
            Int(2),
            List(vec![Int(1), Int(1)]),
        ]);
        let expected = vec![1, 1, 2, 1, 1];
        let ret = check(&mut ni);
        assert_eq!(ret, expected);
    }
    {
        let mut ni = NestedIterator::new(vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])]);
        let expected = vec![1, 4, 6];
        let ret = check(&mut ni);
        assert_eq!(ret, expected);
    }
}
