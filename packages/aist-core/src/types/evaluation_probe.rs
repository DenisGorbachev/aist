use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[derive(Default, Debug)]
pub struct EvaluationProbe {
    pub visited: AtomicBool,
    pub repeated: AtomicBool,
}

impl EvaluationProbe {
    pub fn evaluate<T>(&self, expression: impl FnOnce() -> T) -> T {
        self.repeated
            .fetch_or(self.visited.swap(true, Relaxed), Relaxed);
        expression()
    }

    pub fn once(&self) -> bool {
        self.visited.load(Relaxed) && !self.repeated.load(Relaxed)
    }

    pub fn untouched(&self) -> bool {
        !self.visited.load(Relaxed)
    }
}
