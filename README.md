# Baby's First GC

This is two translations of the simple mark-sweep GC described in [Baby's First Garbage Collector][1]
into Rust. One version uses `unsafe` while the other does not. The unsafe GC is a close to 
literal translation of the original C, with the notable exceptions that it uses `NonNull<T>` 
pointers instead of raw pointers, and stores pointers to all live objects in a `Vec` instead
of using an intrusive linked list. The safe GC is implemented by allocating all objects in a
`Vec` owned by the VM and using indices into the `Vec` as pointers. As a result, all 
operations can be done using only safe Rust.

[1]: http://journal.stuffwithstuff.com/2013/12/08/babys-first-garbage-collector/

Preliminary benchmarking has shown the safe GC to be about 2-3x *faster* than the unsafe GC.
Although these are, of course, microbenchmarks of naively-written toy VMs, this result should
still give some pause. It may be possible to write a production quality, memory-safe GC in Rust,
but the only way to be sure would be to actually do it.
