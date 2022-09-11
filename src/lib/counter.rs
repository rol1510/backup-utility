use std::ops::{Add, AddAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Counter<T> {
    pub included: T,
    pub excluded: T,
}

impl<T: Add + Add<Output = T> + Copy + Default> Counter<T> {
    pub fn new() -> Counter<T> {
        Counter {
            ..Default::default()
        }
    }

    pub fn sum(&self) -> T {
        self.included + self.excluded
    }
}

impl<T: Add + Add<Output = T>> Add<Counter<T>> for Counter<T> {
    type Output = Counter<T>;
    fn add(self, rhs: Counter<T>) -> Counter<T> {
        Counter {
            included: self.included + rhs.included,
            excluded: self.excluded + rhs.excluded,
        }
    }
}

impl<T: AddAssign> AddAssign<Counter<T>> for Counter<T> {
    fn add_assign(&mut self, rhs: Counter<T>) {
        self.included += rhs.included;
        self.excluded += rhs.excluded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! new_counter {
        ($a:expr, $b:expr) => {
            Counter {
                excluded: $a,
                included: $b,
            }
        };
    }

    #[test]
    fn test_default() {
        macro_rules! test {
            ($type:ty, $val:expr) => {
                let c = Counter::<$type>::new();
                assert_eq!(c.excluded, $val);
                assert_eq!(c.included, $val);
            };
        }

        test!(i32, 0);
        test!(u32, 0);

        test!(i64, 0);
        test!(u64, 0);

        test!(i128, 0);
        test!(u128, 0);

        test!(f32, 0.0);
        test!(f64, 0.0);
    }

    #[test]
    fn test_add() {
        let a = new_counter!(10, -5);
        let b = new_counter!(20, 10);
        assert_eq!(new_counter!(30, 5), a + b);
    }

    #[test]
    fn test_add_assign() {
        let mut a = new_counter!(10, -5);
        let b = new_counter!(20, 10);

        a += b;

        assert_eq!(new_counter!(30, 5), a);
        assert_eq!(new_counter!(20, 10), b);
    }

    #[test]
    fn test_sum() {
        let a = new_counter!(15, 5);
        assert_eq!(a.sum(), 20);
    }
}
