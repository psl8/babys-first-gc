use c_gc::c_gc::*;

#[test]
fn test_c_gc() {
    unsafe {
        let vm = newVM();

        for i in 0..64 {
            pushInt(vm, i);
        }
        for _ in 0..63 {
            pushPair(vm);
        }

        assert_ne!((*vm).numObjects, 0);
        assert!(!(*vm).firstObject.is_null());

        pop(vm);

        gc(vm);

        assert_eq!((*vm).numObjects, 0);
        assert!((*vm).firstObject.is_null());

        freeVM(vm);
    }
}
