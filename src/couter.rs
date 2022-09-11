use std::ops::{Add, AddAssign};

#[derive(Debug, Default, Clone, Copy)]
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
