use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use js_runtime::EvalExecutor;

async fn do_eval_expr(executor: &EvalExecutor, expr: String) {
    executor.eval::<i32>(expr).await.unwrap();
}

fn tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

fn eval_expr(c: &mut Criterion) {
    let runtime = tokio_runtime();
    let executor = EvalExecutor::builder().build().unwrap();

    let mut group = c.benchmark_group("eval_expr");

    let exprs = vec![
        "1",
        "Promise.resolve(1)",
        "1+1",
        "Promise.resolve(1+1)",
        "(async () => 1+1)()",
    ];
    for expr in exprs.iter() {
        group.bench_with_input(BenchmarkId::from_parameter(expr), expr, |b, expr| {
            b.to_async(&runtime)
                .iter(|| do_eval_expr(&executor, expr.to_string()))
        });
    }
}

criterion_group!(benches, eval_expr);
criterion_main!(benches);
