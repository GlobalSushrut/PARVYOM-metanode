use bpi_enc::{CanonicalCbor, CanonicalMap, TestStruct, domain_hash, domains};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_cbor_encode(c: &mut Criterion) {
    let test_data = TestStruct {
        height: 12345,
        hash: [0xab; 32],
        metadata: {
            let mut map = CanonicalMap::new();
            for i in 0..100 {
                map.insert(format!("key_{:03}", i), format!("value_{}", i));
            }
            map
        },
    };

    c.bench_function("cbor_encode", |b| {
        b.iter(|| CanonicalCbor::encode(black_box(&test_data)))
    });
}

fn bench_cbor_decode(c: &mut Criterion) {
    let test_data = TestStruct {
        height: 12345,
        hash: [0xab; 32],
        metadata: CanonicalMap::new(),
    };
    let encoded = CanonicalCbor::encode(&test_data).unwrap();

    c.bench_function("cbor_decode", |b| {
        b.iter(|| {
            let _: TestStruct = CanonicalCbor::decode(black_box(&encoded)).unwrap();
        })
    });
}

fn bench_domain_hash(c: &mut Criterion) {
    let data = vec![0xab; 1024];

    c.bench_function("domain_hash_1kb", |b| {
        b.iter(|| domain_hash(black_box(domains::HEADER_HASH), black_box(&data)))
    });
}

criterion_group!(benches, bench_cbor_encode, bench_cbor_decode, bench_domain_hash);
criterion_main!(benches);
