struct Spreadsheet {
    table: Vec<Vec<i32>>,
}

enum Formula {
    Value(i32),
    Index(usize, usize),
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            table: vec![vec![0; 26]; rows as usize],
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        match Self::cell_to_index(&cell).0 {
            Formula::Index(row, col) => self.table[row][col] = value,
            _ => unreachable!("never reach here"),
        }
    }

    fn reset_cell(&mut self, cell: String) {
        match Self::cell_to_index(&cell).0 {
            Formula::Index(row, col) => self.table[row][col] = 0,
            _ => unreachable!("never reach here"),
        }
    }

    fn get_value(&self, formula: String) -> i32 {
        let (v1, pos1) = match Self::cell_to_index(&formula[1..]) {
            (Formula::Index(row, col), pos) => (self.table[row][col], pos),
            (Formula::Value(v), pos) => (v, pos),
        };
        let v2 = match Self::cell_to_index(&formula[pos1 + 2..]).0 {
            Formula::Index(row, col) => self.table[row][col],
            Formula::Value(v) => v,
        };

        v1 + v2
    }

    fn cell_to_index(cell: &str) -> (Formula, usize) {
        fn is_digit(b: u8) -> bool {
            matches!(
                b,
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9'
            )
        }

        let bs: Vec<_> = cell.bytes().collect();
        let len = bs.len();

        if is_digit(bs[0]) {
            let mut v = 0;
            for (i, b) in bs.into_iter().enumerate() {
                if is_digit(b) {
                    v = 10 * v + (b - b'0') as i32;
                } else {
                    return (Formula::Value(v), i);
                }
            }

            (Formula::Value(v), len)
        } else {
            let column = (bs[0] - b'A') as usize;
            let mut row = 0;
            for (i, b) in bs.into_iter().enumerate().skip(1) {
                if is_digit(b) {
                    row = 10 * row + (b - b'0') as usize;
                } else {
                    return (Formula::Index(row - 1, column), i);
                }
            }

            (Formula::Index(row - 1, column), len)
        }
    }
}

fn main() {
    let mut s = Spreadsheet::new(3);
    assert_eq!(s.get_value("=5+7".to_string()), 12);
    s.set_cell("A1".to_string(), 10);
    assert_eq!(s.get_value("=A1+6".to_string()), 16);
    s.set_cell("B2".to_string(), 15);
    assert_eq!(s.get_value("=A1+B2".to_string()), 25);
    s.reset_cell("A1".to_string());
    assert_eq!(s.get_value("=A1+B2".to_string()), 15);
}

#[test]
fn test() {
    {
        let mut s = Spreadsheet::new(3);
        assert_eq!(s.get_value("=5+7".to_string()), 12);
        s.set_cell("A1".to_string(), 10);
        assert_eq!(s.get_value("=A1+6".to_string()), 16);
        s.set_cell("B2".to_string(), 15);
        assert_eq!(s.get_value("=A1+B2".to_string()), 25);
        s.reset_cell("A1".to_string());
        assert_eq!(s.get_value("=A1+B2".to_string()), 15);
    }
}
