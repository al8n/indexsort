use criterion::{Criterion, criterion_group, criterion_main};

use indexsort::IndexSort;

fn is_sorted_small(c: &mut Criterion) {
	let mut data = (0..10_000)
		.map(|_| rand::random::<isize>())
		.collect::<Vec<_>>();
	IndexSort::sort(&mut data);

	c.bench_function("is_sorted(10k)", |b| {
		b.iter(|| {
			assert!(IndexSort::is_sorted(&data));
		});
	});

	c.bench_function("is_sorted2(10k)", |b| {
		b.iter(|| {
			assert!(IndexSort::is_sorted2(&data));
		});
	});
}

fn is_sorted_large(c: &mut Criterion) {
	let mut data = (0..100_000_000)
		.map(|_| rand::random::<isize>())
		.collect::<Vec<_>>();
	IndexSort::sort(&mut data);

	c.bench_function("is_sorted(100m)", |b| {
		b.iter(|| {
			assert!(IndexSort::is_sorted(&data));
		});
	});

	c.bench_function("is_sorted2(100m)", |b| {
		b.iter(|| {
			assert!(IndexSort::is_sorted2(&data));
		});
	});
}

criterion_group!(
    benches,
    is_sorted_small,
	is_sorted_large
);
criterion_main!(benches);