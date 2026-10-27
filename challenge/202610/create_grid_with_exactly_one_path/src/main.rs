fn create_grid(m: i32, n: i32) -> Vec<String> {
    let mut ret = vec![];
    let (m, n) = (m as usize, n as usize);

    let mut s = String::new();
    for _ in 0..n {
        s.push('.');
    }
    ret.push(s);

    for _ in 1..m {
        let mut s = String::new();
        for _ in 0..(n - 1) {
            s.push('#');
        }
        s.push('.');
        ret.push(s);
    }

    ret
}

fn main() {
    let ret = create_grid(2, 3);
    println!("ret={ret:?}");
}
