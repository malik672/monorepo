use commonware_cryptography::{Bls12381, Scheme};
use criterion::{criterion_group, BatchSize, Criterion};
use rand::thread_rng;
use std::hint::black_box;

fn benchmark_signature_generation(c: &mut Criterion) {
    let namespace = b"namespace";
    let msg = b"hello";
    c.bench_function(
        &format!(
            "{}/ns_len={} msg_len={}",
            module_path!(),
            namespace.len(),
            msg.len()
        ),
        |b| {
            b.iter_batched(
                || Bls12381::new(&mut thread_rng()),
                |mut signer| {
                    black_box(signer.sign(Some(namespace), msg));
                },
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(benches, benchmark_signature_generation);
