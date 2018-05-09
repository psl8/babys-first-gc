#[allow(non_snake_case, non_upper_case_globals)]
mod c_gc_bindings;

pub use self::c_gc_bindings::{STACK_MAX, ObjectType_OBJ_INT, ObjectType_OBJ_PAIR};
use self::c_gc_bindings::*;

pub struct Vm(*mut VM);

impl Vm {
    pub fn new() -> Self {
        unsafe {
            Vm(newVM())
        }
    }

    pub fn push_int(&mut self, i: i32) {
        unsafe {
            pushInt(self.0, i as ::std::os::raw::c_int);
        }
    }

    pub fn push_pair(&mut self) {
        unsafe {
            pushPair(self.0);
        }
    }

    pub fn gc(&mut self) {
        unsafe {
            gc(self.0);
        }
    }

    pub fn drop(&mut self) {
        unsafe {
            pop(self.0);
        }
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        unsafe {
            freeVM(self.0);
        }
    }
}

use std::ops::{Deref, DerefMut};
impl Deref for Vm {
    type Target = VM;

    fn deref(&self) -> &VM {
        unsafe {
            &*self.0
        }
    }
}
impl DerefMut for Vm {
    fn deref_mut(&mut self) -> &mut VM {
        unsafe {
            &mut *self.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_c_gc() {
        let mut vm = Vm::new();

        for i in 0..64 {
            vm.push_int(i);
        }
        for _ in 0..63 {
            vm.push_pair();
        }

        assert_ne!(vm.numObjects, 0);
        assert!(!vm.firstObject.is_null());

        vm.drop();

        vm.gc();

        assert_eq!(vm.numObjects, 0);
        assert!(vm.firstObject.is_null());
    }
}
