use criterion::{ black_box, criterion_group, criterion_main, Criterion };
use globset::{ Glob, GlobSetBuilder };
use regex::Regex;

fn globset_match(c: &mut Criterion) {
  // 可在外部执行时设置
  //let mut c = Criterion::default()
  //  .sample_size(100) // 设置样本数量为100
  //  .measurement_time(Duration::from_secs(10)) // 设置测量时间10秒
  //  .warm_up_time(Duration::from_secs(5)); // 设置预热时间5秒

  let mut builder = GlobSetBuilder::new();
  builder.add(Glob::new("/public/**").unwrap());
  builder.add(Glob::new("/api/login").unwrap());
  let globset = builder.build().unwrap();

  c.bench_function("globset match", |b| {
    b.iter(|| {
      black_box(globset.is_match("/public/css/style.css"));
      black_box(globset.is_match("/api/login"));
      black_box(globset.is_match("/private/data"));
    })
  });
}

fn regex_match(c: &mut Criterion) {
  let pattern = "^/public/.*$|^/api/login$";
  let re = Regex::new(pattern).unwrap();

  c.bench_function("regex match", |b| {
    b.iter(|| {
      black_box(re.is_match("/public/css/style.css"));
      black_box(re.is_match("/api/login"));
      black_box(re.is_match("/private/data"));
    })
  });
}

// 定义基准测试组
criterion_group!(benches, globset_match, regex_match);
// Rust 基准测试主入口
criterion_main!(benches);
