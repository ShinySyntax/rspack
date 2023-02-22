#![recursion_limit = "256"]
use std::{hint::black_box, path::PathBuf, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use mimalloc_rust::GlobalMiMalloc;
use rspack_core::Compiler;
use rspack_fs::AsyncNativeFileSystem;
use rspack_testing::apply_from_fixture;
use xshell::{cmd, Shell};

#[cfg(all(not(all(target_os = "linux", target_arch = "aarch64", target_env = "musl"))))]
#[global_allocator]
static GLOBAL: GlobalMiMalloc = GlobalMiMalloc;

async fn bench(cur_dir: &PathBuf) {
  let (options, plugins) = apply_from_fixture(cur_dir);
  let mut compiler = Compiler::new(options, plugins, AsyncNativeFileSystem);

  compiler
    .build()
    .await
    .unwrap_or_else(|_| panic!("failed to compile in fixtrue {cur_dir:?}"));
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("criterion_benchmark");
  group.sample_size(100);
  group.measurement_time(Duration::new(10, 0));
  let sh = Shell::new().expect("TODO:");
  println!("{:?}", sh.current_dir());
  sh.change_dir(PathBuf::from(env!("CARGO_WORKSPACE_DIR")));
  cmd!(sh, "cargo xtask copy_three").run().expect("TODO:");
  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("TODO:");
  generate_bench!(ten_copy_of_threejs, "three", group, rt);
  group.finish();

  // High cost benchmark
  // sample count reduce to 30
  let mut group = c.benchmark_group("high_cost_benchmark");
  group.sample_size(30);
  group.measurement_time(Duration::new(180, 0));
  let sh = Shell::new().expect("TODO:");
  println!("{:?}", sh.current_dir());
  sh.change_dir(PathBuf::from(env!("CARGO_WORKSPACE_DIR")));
  cmd!(sh, "cargo xtask three_production_config")
    .run()
    .expect("TODO:");
  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .expect("TODO:");
  generate_bench!(ten_copy_of_threejs_production, "three", group, rt);
  group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[macro_export]
macro_rules! generate_bench {
  ($id: ident, $dir: expr, $c: ident, $rt: ident) => {
    let $id: PathBuf = concat!(env!("CARGO_MANIFEST_DIR"), "/../../benchcases/", $dir).into();
    $c.bench_function(stringify!($id), |b| {
      b.to_async(&$rt).iter(|| black_box(bench(&$id)))
    });
  };
}
