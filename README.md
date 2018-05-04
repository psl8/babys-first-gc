# Baby's First GC

This is two translations of the simple mark-sweep GC described in [Baby's First Garbage Collector][1]
into Rust. One version uses `unsafe` while the other does not. The unsafe GC is a close to 
literal translation of the original C, with the notable exception that it uses `NonNull<T>` 
pointers instead of raw pointers. The safe GC is implemented by allocating all objects in a
`Vec` owned by the VM and using indices into the `Vec` as pointers. As a result, all 
operations can be done using only safe Rust.

[1]: http://journal.stuffwithstuff.com/2013/12/08/babys-first-garbage-collector/
