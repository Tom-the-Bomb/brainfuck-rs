//! benchmarks for brainfuck operations

use std::{io::Cursor, time::Duration};
use brainfuck_exe::Brainfuck;
use criterion::{
    criterion_group,
    criterion_main,
    black_box,
    Criterion,
};

fn brainfuck_bench(c: &mut Criterion) {
    let mut c = c.benchmark_group("brainfuck_operations");
    c.warm_up_time(Duration::from_millis(1500));

    let mut brainfuck = black_box(
        Brainfuck::new("
        >++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
        +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
        ]<+.
        ")
            .with_output(Cursor::new(Vec::new()))
    );
    c.bench_function(
        "(output) brainfuck_hello_world",
        |b| b.iter(|| brainfuck.execute().ok())
    );

    brainfuck = black_box(
        Brainfuck::new(",+>,++>,+>,++>,+>")
            .with_input(Cursor::new("12345".as_bytes().to_vec()))
    );
    c.bench_function(
        "(input) brainfuck_input",
        |b| b.iter(|| brainfuck.execute().ok())
    );

    c.finish();
}

criterion_group!(benches, brainfuck_bench);
criterion_main!(benches);