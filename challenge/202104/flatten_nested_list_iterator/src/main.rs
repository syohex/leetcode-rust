pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    values: Vec<i32>,
    pos: usize,
}

fn flatten(values: &mut Vec<i32>, v: &Vec<NestedInteger>) {
    for value in v.iter() {
        match &value {
            NestedInteger::Int(i) => values.push(*i),
            NestedInteger::List(lst) => flatten(values, lst),
        }
    }
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut values = vec![];
        flatten(&mut values, &nested_list);
        Self { values, pos: 0 }
    }

    fn next(&mut self) -> i32 {
        let ret = self.values[self.pos];
        self.pos += 1;
        ret
    }

    fn has_next(&self) -> bool {
        self.pos < self.values.len()
    }
}

fn main() {
    let lst = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];

    let mut iter = NestedIterator::new(lst);
    while iter.has_next() {
        println!("value={}", iter.next());
    }
}

#[test]
fn test_nested_iterator() {
    {
        let lst = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];

        let mut iter = NestedIterator::new(lst);
        assert!(iter.has_next());
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 2);
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 1);
        assert!(!iter.has_next())
    }
    {
        let lst = vec![NestedInteger::List(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ])];

        let mut iter = NestedIterator::new(lst);
        assert!(iter.has_next());
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 4);
        assert_eq!(iter.next(), 6);
        assert!(!iter.has_next());
    }
}
