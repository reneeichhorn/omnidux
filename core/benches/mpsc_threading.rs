use criterion::Criterion;
use criterion::black_box;

use omnidux_core::scheduler::strategy::{ScheduleStrategy, take_first};
use omnidux_core::task::{TaskHandler, Task};

pub fn mpsc_threading(c: &mut Criterion) {
    c.bench_function("thread[2] take_first", |b| b.iter(|| false));
}

criterion_group!(benches, mpsc_threading);
criterion_main!(benches);