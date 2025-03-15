use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ghayb::{circuit, constraint, prove, verify, witness, FieldElement, PRIME};

fn bench_prove(c: &mut Criterion) {
    let ckt = circuit! {
        [x] -> [out],
        constraints = [
            constraint!((w@Witness + x@Input) -> out@Output)
        ]
    };

    let w = witness! {
        w = 5,
        x = 10
    };

    c.bench_function("Prover - Simple Addition", |b| {
        b.iter(|| prove!(black_box(ckt.clone()), black_box(w.clone())))
    });
}

fn bench_verify(c: &mut Criterion) {
    let ckt = circuit! {
        [x] -> [out],
        constraints = [
            constraint!((w@Witness + x@Input) -> out@Output)
        ]
    };

    let w = witness! {
        w = 5,
        x = 10
    };

    let proof = prove!(ckt.clone(), w);

    c.bench_function("Verifier - Simple Addition", |b| {
        b.iter(|| {
            verify!(
                black_box(ckt.clone()),
                [FieldElement::new(10, PRIME)],
                black_box(proof.clone())
            )
        })
    });
}

fn bench_e2e(c: &mut Criterion) {
    let ckt = circuit! {
        [x] -> [out],
        constraints = [
            constraint!((w@Witness + x@Input) -> out@Output)
        ]
    };

    let w = witness! {
        w = 5,
        x = 10
    };

    c.bench_function("End-to-End - Prove and Verify", |b| {
        b.iter(|| {
            let proof = prove!(black_box(ckt.clone()), black_box(w.clone()));
            let result = verify!(
                black_box(ckt.clone()),
                [FieldElement::new(10, PRIME)],
                black_box(proof)
            );
            assert!(result); // Ensure proof is valid
        })
    });
}

criterion_group!(benches, bench_prove, bench_verify, bench_e2e);
criterion_main!(benches);
