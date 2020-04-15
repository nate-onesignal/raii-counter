use futures_intrusive::sync::ManualResetEvent;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct Counter {
    counter: Arc<AtomicUsize>,
    event: Arc<ManualResetEvent>,
}

impl Counter {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(count)),
            event: Arc::new(ManualResetEvent::new(false)),
        }
    }

    pub(crate) async fn wait_for_empty(&self) {
        while self.get() != 0 {
            self.event.wait().await;
        }
    }

    #[inline]
    pub(crate) fn fetch_add(&self, amount: usize) {
        let count = self.counter.fetch_add(amount, Ordering::AcqRel);
        if count + amount == 0 {
            self.event.set();
        } else if self.event.is_set() {
            self.event.reset();
        }
    }

    #[inline]
    pub(crate) fn fetch_sub(&self, amount: usize) {
        let count = self.counter.fetch_sub(amount, Ordering::AcqRel);
        if count - amount == 0 {
            self.event.set();
        } else if self.event.is_set() {
            self.event.reset();
        }
    }

    /// This method is inherently racey. Assume the count will have changed once
    /// the value is observed.
    #[inline]
    pub(crate) fn get(&self) -> usize {
        self.counter.load(Ordering::Acquire)
    }
}
