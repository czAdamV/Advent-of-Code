use scan_fmt::scan_fmt;
use std::io::stdin;

struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Interval {
        Interval { start, end }
    }

    fn len(&self) -> i64 {
        self.end - self.start
    }

    fn contains(&self, val: i64) -> bool {
        self.start <= val && val <= self.end
    }
}

pub fn main() -> Result<(), String> {
    let mut redundancies = 0;
    let mut overlaps = 0;

    for line in stdin().lines().map(|x| x.unwrap()).filter(|x| x != "") {
        let scan = scan_fmt!(&line, "{}-{},{}-{}", i64, i64, i64, i64);
        let (s1, e1, s2, e2) = scan.unwrap();

        let mut first = Interval::new(s1, e1);
        let mut second = Interval::new(s2, e2);

        if first.len() < second.len() {
            (first, second) = (second, first);
        }

        if first.contains(second.start) && first.contains(second.end) {
            redundancies += 1;
        }

        if first.contains(second.start) || first.contains(second.end) {
            overlaps += 1;
        }
    }

    println!("{redundancies}");
    println!("{overlaps}");

    Ok(())
}
