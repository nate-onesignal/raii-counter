# raii-counter-futures

Fork of DarrenTsung's [raii-counter](https://crates.io/crates/raii-counter)
library that allows you to wait asynchronously for the count to reach zero.

## Demo

```rust
use raii_counter_futures::WeakCounter;
use std::time::Duration;

let weak = Arc::new(WeakCounter::new());
assert_eq!(counter.count(), 0);

let weak2 = Arc::clone(&weak);
tokio::spawn(async move {
    let counter1 = weak2.spawn_upgrade();
    let counter2 = weak2.spawn_upgrade();

    tokio::time::delay_for(Duration::from_secs(2)).await;
});

// Give the weak counters a chance to spawn, in a real workload you would not
// want to start waiting immediately after your async tasks spawn.
tokio::time::delay_for(Duration::from_millis(500)).await;

// This will not complete until the 2 second delay in the async task has finished.
counter.wait_for_empty().await;
```
