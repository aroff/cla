//! Benchmark tests for CLA algorithm performance

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::Array1;
use rustcla::Cla;

fn generate_portfolio(n: usize) -> (Array1<f64>, ndarray::Array2<f64>, Array1<f64>, Array1<f64>) {
    use ndarray::Array2;
    // Generate random but valid portfolio data
    let mean = Array1::from_iter((0..n).map(|i| 0.1 + (i as f64) * 0.01));

    // Generate positive definite covariance matrix
    let mut covariance = Array2::<f64>::zeros((n, n));
    for i in 0..n {
        covariance[(i, i)] = 0.01 + (i as f64) * 0.001;
        for j in (i + 1)..n {
            let cov = 0.001 * (1.0 - (i as f64 - j as f64).abs() / n as f64);
            covariance[(i, j)] = cov;
            covariance[(j, i)] = cov;
        }
    }

    let lower_bounds = Array1::zeros(n);
    let upper_bounds = Array1::ones(n);

    (mean, covariance, lower_bounds, upper_bounds)
}

fn bench_cla_10_assets(c: &mut Criterion) {
    let n = 10;
    let (mean, covariance, lower_bounds, upper_bounds) = generate_portfolio(n);
    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    let b_val = b.clone();
    c.bench_function("cla_10_assets", |bencher| {
        bencher.iter(|| {
            black_box(Cla::new(
                mean.clone(),
                covariance.clone(),
                lower_bounds.clone(),
                upper_bounds.clone(),
                a.clone(),
                b_val.clone(),
            ))
        });
    });
}

fn bench_cla_100_assets(c: &mut Criterion) {
    let n = 100;
    let (mean, covariance, lower_bounds, upper_bounds) = generate_portfolio(n);
    let a = ndarray::Array2::from_shape_vec((1, n), vec![1.0; n]).unwrap();
    let b = Array1::from_vec(vec![1.0]);

    let b_val = b.clone();
    c.bench_function("cla_100_assets", |bencher| {
        bencher.iter(|| {
            black_box(Cla::new(
                mean.clone(),
                covariance.clone(),
                lower_bounds.clone(),
                upper_bounds.clone(),
                a.clone(),
                b_val.clone(),
            ))
        });
    });
}

criterion_group!(benches, bench_cla_10_assets, bench_cla_100_assets);
criterion_main!(benches);
