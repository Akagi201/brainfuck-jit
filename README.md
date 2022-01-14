# Brainfuck JIT

A toy brainfuck jit compiler

## Run

```bash
cargo build --release
./target/release/bf_inter data/mandelbrot.bf # slow
./target/release/bf_ir data/mandelbrot.bf    # faster
./target/release/bf_jit data/mandelbrot.bf   # fastest, only can run on MacOS and Linux
```

## Brainfuck visualizer

<https://brainfuck-visualizer.herokuapp.com>
