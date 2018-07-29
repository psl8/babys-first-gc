#[macro_use]
extern crate criterion;
extern crate babys_first_gc as gc;

use criterion::{Criterion, Fun};
use gc::{c_gc, safe_gc, unsafe_gc};

fn short_benchmark(c: &mut Criterion) {
    let mut safe_vm = safe_gc::Vm::new();
    let safe_bench = Fun::new("Safe GC", move |b, &num_objects| {
        b.iter(|| {
            for i in 0..num_objects {
                safe_vm.push_int(i);
            }
            for _ in 0..(num_objects - 1) {
                safe_vm.push_pair();
            }

            safe_vm.pop();
            safe_vm.gc();
        })
    });

    let mut unsafe_vm = unsafe_gc::Vm::new();
    let unsafe_bench = Fun::new("Unsafe GC", move |b, &num_objects| {
        b.iter(|| {
            for i in 0..num_objects {
                unsafe_vm.push_int(i);
            }
            for _ in 0..(num_objects - 1) {
                unsafe_vm.push_pair();
            }

            unsafe_vm.pop();
            unsafe_vm.gc();
        })
    });

    let mut c_vm = c_gc::Vm::new();
    let c_bench = Fun::new("C GC", move |b, &num_objects| {
        b.iter(|| {
            for i in 0..num_objects {
                c_vm.push_int(i as i32);
            }
            for _ in 0..(num_objects - 1) {
                c_vm.push_pair();
            }

            c_vm.pop();
            c_vm.gc();
        })
    });

    let functions = vec![safe_bench, unsafe_bench, c_bench];
    c.bench_functions("Short bench", functions, 64);
}

fn long_benchmark(c: &mut Criterion) {
    let mut safe_vm = safe_gc::Vm::new();
    let safe_bench = Fun::new("Safe GC", move |b, &num_objects| {
        b.iter(|| {
            for i in 0..num_objects {
                safe_vm.push_int(i);
            }
            for _ in 0..(num_objects - 1) {
                safe_vm.push_pair();
            }

            safe_vm.pop();
            safe_vm.gc();
        })
    });

    let mut unsafe_vm = unsafe_gc::Vm::new();
    let unsafe_bench = Fun::new("Unsafe GC", move |b, &num_objects| {
        b.iter(|| {
            for i in 0..num_objects {
                unsafe_vm.push_int(i);
            }
            for _ in 0..(num_objects - 1) {
                unsafe_vm.push_pair();
            }

            unsafe_vm.pop();
            unsafe_vm.gc();
        })
    });

    let mut c_vm = c_gc::Vm::new();
    let c_bench = Fun::new("C GC", move |b, &num_objects| {
        b.iter(|| {
            for i in 0..num_objects {
                c_vm.push_int(i as i32);
            }
            for _ in 0..(num_objects - 1) {
                c_vm.push_pair();
            }

            c_vm.pop();
            c_vm.gc();
        })
    });

    let functions = vec![safe_bench, unsafe_bench, c_bench];
    c.bench_functions("Long bench", functions, 1 << 16);
}


criterion_group!(
    benches,
    short_benchmark,
    long_benchmark,
);
criterion_main!(benches);
