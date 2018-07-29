use std::ptr::NonNull;

const INITIAL_GC_THRESHOLD: usize = 32;

#[derive(Clone, Debug)]
pub struct GcPtr<T>(NonNull<T>);

#[derive(Clone, Debug)]
pub struct Object {
    marked: bool,
    obj_type: ObjectType,
}

#[derive(Clone, Debug)]
pub enum ObjectType {
    Int(i64),
    Pair(Pair),
}

#[derive(Clone, Debug)]
pub struct Pair {
    head: Option<GcPtr<Object>>,
    tail: Option<GcPtr<Object>>,
}

impl GcPtr<Object> {
    unsafe fn mark(&mut self) {
        if self.0.as_ref().marked {
            return;
        }

        self.0.as_mut().marked = true;

        if let ObjectType::Pair(ref mut pair) = self.0.as_mut().obj_type {
            if let Some(ref mut head) = pair.head {
                head.mark();
            }

            if let Some(ref mut tail) = pair.tail {
                tail.mark();
            }
        }
    }
}

#[derive(Default)]
pub struct Vm {
    num_objects: usize,
    max_objects: usize,
    objects: Vec<GcPtr<Object>>,
    stack: Vec<GcPtr<Object>>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            num_objects: 0,
            max_objects: INITIAL_GC_THRESHOLD,
            objects: Vec::new(),
            stack: Vec::new(),
        }
    }

    unsafe fn mark_all(&mut self) {
        for object in &mut self.stack {
            object.mark();
        }
    }

    unsafe fn sweep(&mut self) {
        let mut live_objects = Vec::new();

        for object_ptr in &mut self.objects {
            if !object_ptr.0.as_ref().marked {
                let unreached = object_ptr.0.as_mut();
                self.num_objects -= 1;

                // This takes ownership then Drops, deallocating the object
                drop(Box::from_raw(unreached));
            } else {
                object_ptr.0.as_mut().marked = false;
                live_objects.push(object_ptr.clone());
            }
        }

        self.objects = live_objects;
    }

    pub fn gc(&mut self) {
        let num_objects = self.num_objects;

        // This is only safe if we have exclusive access to every
        // object during garbage collection. Since there is no way
        // to get a GcPtr out of a Vm and we have a unique reference
        // to self, this invariant should always hold
        unsafe {
            self.mark_all();
            self.sweep();
        }

        self.max_objects = num_objects * 2;
    }

    fn new_object(&mut self, obj_type: ObjectType) -> GcPtr<Object> {
        if self.num_objects == self.max_objects {
            self.gc();
        }

        let mut box_object = Box::new(Object {
            marked: false,
            obj_type,
        });
        let object = GcPtr(NonNull::new(&mut *box_object).unwrap());
        // Don't deallocate our object
        ::std::mem::forget(box_object);

        self.objects.push(object.clone());

        self.num_objects += 1;
        object
    }

    pub fn push_int(&mut self, val: i64) {
        let obj = self.new_object(ObjectType::Int(val));
        self.stack.push(obj);
    }

    pub fn push_pair(&mut self) {
        let tail = Some(self.pop());
        let head = Some(self.pop());
        let obj = self.new_object(ObjectType::Pair(Pair { head, tail }));
        self.stack.push(obj);
    }

    pub fn pop(&mut self) -> GcPtr<Object> {
        self.stack.pop().expect("Stack underflow!")
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        self.stack = vec![];
        self.gc();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unsafe_gc() {
        let mut vm = Vm::new();

        for i in 0..64 {
            vm.push_int(i);
        }
        for _ in 0..63 {
            vm.push_pair();
        }

        assert_eq!(vm.stack.len(), 1);
        assert_ne!(vm.num_objects, 0);
        assert_ne!(vm.objects.len(), 0);

        vm.pop();

        vm.gc();

        assert_eq!(vm.num_objects, 0);
        assert_eq!(vm.objects.len(), 0);
    }
}
