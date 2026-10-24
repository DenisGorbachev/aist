use std::cell::Cell;

#[derive(Debug)]
pub struct CallProbe<T: Copy> {
    pub last: Cell<Option<T>>,
    pub repeated: Cell<bool>,
}

impl<T: Copy> Default for CallProbe<T> {
    fn default() -> Self {
        Self {
            last: Cell::new(None),
            repeated: Cell::new(false),
        }
    }
}

impl<T: Copy> CallProbe<T> {
    pub fn record(&self, value: T) {
        let repeated = self.last.replace(Some(value)).is_some();
        self.repeated.set(self.repeated.get() || repeated);
    }

    pub fn once(&self) -> bool {
        self.last.get().is_some() && !self.repeated.get()
    }
}
