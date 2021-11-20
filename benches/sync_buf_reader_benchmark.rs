use std::io::{BufReader, Read};

mod util;

use criterion::{criterion_group, criterion_main, Bencher, Criterion};

#[cfg(target_os = "linux")]
use pprof::criterion::{Output, PProfProfiler};

use util::*;

fn bench_read<const C: usize>(bencher: &mut Bencher) {
    let mut buf_reader = BufReader::with_capacity(C, EndlessZeroReader);

    bencher.iter(|| {
        let mut buf = [0; C];
        buf_reader.read(&mut buf).unwrap();
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "std::io::BufReader (capacity = 16) - Read",
        bench_read::<16>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 32) - Read",
        bench_read::<32>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 64) - Read",
        bench_read::<64>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 128) - Read",
        bench_read::<128>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 256) - Read",
        bench_read::<256>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 512) - Read",
        bench_read::<512>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 1024) - Read",
        bench_read::<1024>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 2048) - Read",
        bench_read::<2048>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 4096) - Read",
        bench_read::<4096>,
    );
    c.bench_function(
        "std::io::BufReader (capacity = 8192 (std-default)) - Read",
        bench_read::<8192>,
    );
}

#[cfg(target_os = "linux")]
criterion_group! {
    name = benches;
    config = Criterion::default()
        .with_profiler(
            PProfProfiler::new(100, Output::Flamegraph(None))
        );
    targets = criterion_benchmark
}

#[cfg(not(target_os = "linux"))]
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
