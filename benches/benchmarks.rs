#![feature(test)]
extern crate test;
extern crate babys_first_gc as gc;

use test::*;
use gc::{safe_gc, unsafe_gc};

#[bench]
fn bench_safe_gc(b: &mut Bencher) {
    let mut vm = safe_gc::Vm::new();
    b.iter(|| {
        let num_objects = black_box(64);
        for i in 0..num_objects {
            vm.push_int(i);
        }
        for _ in 0..(num_objects - 1) {
            vm.push_pair();
        }

        vm.drop();
        vm.gc();
    });
}

#[bench]
fn bench_unsafe_gc(b: &mut Bencher) {
    let mut vm = unsafe_gc::Vm::new();
    b.iter(|| {
        let num_objects = black_box(64);
        for i in 0..num_objects {
            vm.push_int(i);
        }
        for _ in 0..(num_objects - 1) {
            vm.push_pair();
        }

        vm.drop();
        vm.gc();
    });
}

#[bench]
fn long_bench_safe_gc(b: &mut Bencher) {
    let mut vm = safe_gc::Vm::new();
    b.iter(|| {
        let num_objects = black_box(1 << 16);
        for i in 0..num_objects {
            vm.push_int(i);
        }
        for _ in 0..(num_objects - 1) {
            vm.push_pair();
        }

        vm.drop();
        vm.gc();
    });
}

#[bench]
fn long_bench_unsafe_gc(b: &mut Bencher) {
    let mut vm = unsafe_gc::Vm::new();
    b.iter(|| {
        let num_objects = black_box(1 << 16);
        for i in 0..num_objects {
            vm.push_int(i);
        }
        for _ in 0..(num_objects - 1) {
            vm.push_pair();
        }

        vm.drop();
        vm.gc();
    });
}

use gc::c_gc::c_gc;
#[bench]
fn bench_c_gc(b: &mut Bencher) {
    unsafe {
        let vm = c_gc::newVM();

        b.iter(|| {
            let num_objects = black_box(64);
            for i in 0..num_objects {
                c_gc::pushInt(vm, i);
            }
            for _ in 0..(num_objects - 1) {
                c_gc::pushPair(vm);
            }

            c_gc::pop(vm);
            c_gc::gc(vm);
        });

        c_gc::freeVM(vm);
    }
}

#[bench]
fn long_bench_c_gc(b: &mut Bencher) {
    unsafe {
        let vm = c_gc::newVM();

        b.iter(|| {
            let num_objects = black_box(1 << 16);
            for i in 0..num_objects {
                c_gc::pushInt(vm, i);
            }
            for _ in 0..(num_objects - 1) {
                c_gc::pushPair(vm);
            }

            c_gc::pop(vm);
            c_gc::gc(vm);
        });

        c_gc::freeVM(vm);
    }
}
