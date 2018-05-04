use std::ptr::NonNull;

const INITIAL_GC_THRESHOLD: usize = 32;

#[derive(Clone, Copy)]
struct GcPtr<T>(NonNull<T>);

#[derive(Clone, Copy)]
pub struct Object {
    next: Option<GcPtr<Object>>,
    marked: bool,
    obj_type: ObjectType,
}

#[derive(Clone, Copy)]
pub enum ObjectType {
    Int(i64),
    Pair(Pair),
}

#[derive(Clone, Copy)]
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

        if let ObjectType::Pair(pair) = self.0.as_mut().obj_type {
            if let Some(mut head) = pair.head {
                head.mark();
            }
            if let Some(mut tail) = pair.tail {
                tail.mark();
            }
        }
    }
}

pub struct Vm {
    num_objects: usize,
    max_objects: usize,
    first_object: Option<GcPtr<Object>>,
    stack: Vec<GcPtr<Object>>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            num_objects: 0,
            max_objects: INITIAL_GC_THRESHOLD,
            first_object: None,
            stack: Vec::new(),
        }
    }

    unsafe fn mark_all(&mut self) {
        for object in self.stack.iter_mut() {
            object.mark();
        }
    }

    unsafe fn sweep(&mut self) {
        let mut object = self.first_object;
        while let Some(mut obj_ptr) = object {
            if !obj_ptr.0.as_ref().marked {
                let unreached = obj_ptr.0.as_mut();

                object = unreached.next;
                self.num_objects -= 1;

                // This takes ownership then Drops, deallocating the object
                drop(Box::from_raw(unreached));
            } else {
                obj_ptr.0.as_mut().marked = false;
                object = obj_ptr.0.as_ref().next;
            }
        }
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
            next: self.first_object,
            marked: false,
            obj_type,
        });
        let object = GcPtr(NonNull::new(&mut *box_object).unwrap());
        self.first_object = Some(object);

        self.num_objects += 1;
        object
    }

    pub fn push_int(&mut self, val: i64) {
        let obj = self.new_object(ObjectType::Int(val));
        self.stack.push(obj);
    }

    pub fn push_pair(&mut self) {
        let tail = Some(self.stack.pop().expect("Stack underflow!"));
        let head = Some(self.stack.pop().expect("Stack underflow!"));
        let obj = self.new_object(ObjectType::Pair( Pair { head, tail }));
        self.stack.push(obj);
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        self.stack = vec![];
        self.gc();
    }
}
