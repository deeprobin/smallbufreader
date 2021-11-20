use std::io::Read;

mod util;

use criterion::{criterion_group, criterion_main, Bencher, Criterion};

#[cfg(target_os = "linux")]
use pprof::criterion::{Output, PProfProfiler};

use smallbufreader::SmallBufReader;
use util::*;

fn bench_read<const S: usize>(bencher: &mut Bencher) {
    let mut small_buf_reader = SmallBufReader::<EndlessZeroReader, [u8; S]>::new(EndlessZeroReader);

    bencher.iter(|| {
        let mut buf = [0; S];
        small_buf_reader.read(&mut buf).unwrap();
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 16]> - Read",
        bench_read::<16>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 32]> - Read",
        bench_read::<32>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 64]> - Read",
        bench_read::<64>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 128]> - Read",
        bench_read::<128>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 256]> - Read",
        bench_read::<256>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 512]> - Read",
        bench_read::<512>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 1024]> - Read",
        bench_read::<1024>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 2048]> - Read",
        bench_read::<2048>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 4096]> - Read",
        bench_read::<4096>,
    );
    c.bench_function(
        "smallbufreader::SmallBufReader<[u8; 8192]> - Read",
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
