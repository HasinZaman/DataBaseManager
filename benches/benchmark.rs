use std::{time::Duration};

use DataBaseManager::backend::{data_base::DataBase, sql::SQL};
use criterion::{black_box, criterion_group, criterion_main, Criterion};


pub fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("snap_shot loading", |b| {
        b.iter(|| {
            
            
            let db = DataBase::from_env().unwrap();

            let _result = db.rollback(
                black_box(
                    SQL::from_file(
                        "C:\\Users\\hasin\\Documents\\Portfolio\\DataBaseManager\\snap_shots\\snap_shot_1672406911.sql"
                    ).unwrap()
                )
            );
        });
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(20))
        .measurement_time(Duration::from_secs(200))
        .sample_size(70);
    targets = criterion_benchmark
}
criterion_main!(benches);