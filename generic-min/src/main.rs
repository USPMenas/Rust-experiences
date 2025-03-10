/// A basic trait providing "partial" partial order.
trait LessThan {
    /// Return true if self is less than other.
    fn less_than(&self, other: &Self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Citation {
    author: &'static str,
    year: u32,
}

impl LessThan for Citation {
    fn less_than(&self, other: &Self) -> bool {
        if self.author < other.author {
            true
        } else if self.author > other.author {
            false
        } else {
            self.year < other.year
        }
    }
}

impl LessThan for i32 {
    fn less_than(&self, other: &Self) -> bool {
        self < other
    }
}

// TODO: implement the `min` function used in `main`.

fn min<T: LessThan>(a: T, b: T) -> T {
    if a.less_than(&b) {
        a
    } else {
        b
    }
}

fn main() {
    let cit1 = Citation { author: "Shapiro", year: 2011 };
    let cit2 = Citation { author: "Baumann", year: 2010 };
    let cit3 = Citation { author: "Baumann", year: 2019 };
    debug_assert_eq!(min(cit1, cit2), cit2);
    debug_assert_eq!(min(cit2, cit3), cit2);
    debug_assert_eq!(min(cit1, cit3), cit3);
    let val1 = 42;
    let val2 = -34;
    let val3 = 33;
    debug_assert_eq!(min(val1, val2), val2);
    debug_assert_eq!(min(val2, val3), val2);
    debug_assert_eq!(min(val1, val3), val3);
    println!("Congratulation, it works!");
}