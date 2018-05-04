/* http://journal.stuffwithstuff.com/2013/12/08/babys-first-garbage-collector/ */

use std::ptr::NonNull;

const INITIAL_GC_THRESHOLD: usize = 32;

#[derive(Clone, Copy)]
struct GcPtr<T>(Option<NonNull<T>>);

#[derive(Clone, Copy)]
pub struct Object {
    next: GcPtr<Object>,
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
    head: GcPtr<Object>,
    tail: GcPtr<Object>,
}

impl GcPtr<Object> {
    unsafe fn mark(&mut self) {
        if let Some(mut obj_ptr) = self.0 {
            if obj_ptr.as_ref().marked {
                return;
            }

            obj_ptr.as_mut().marked = true;

            if let ObjectType::Pair(mut pair) = obj_ptr.as_mut().obj_type {
                pair.head.mark();
                pair.tail.mark();
            }
        }
    }
}

pub struct Vm {
    num_objects: usize,
    max_objects: usize,
    first_object: GcPtr<Object>,
    stack: Vec<GcPtr<Object>>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            num_objects: 0,
            max_objects: INITIAL_GC_THRESHOLD,
            first_object: GcPtr(None),
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
        while let Some(mut obj_ptr) = object.0 {
            if !obj_ptr.as_ref().marked {
                let unreached = obj_ptr.as_mut();

                object = unreached.next;
                self.num_objects -= 1;

                // This takes ownership then Drops, deallocating the object
                drop(Box::from_raw(unreached));
            } else {
                obj_ptr.as_mut().marked = false;
                object = obj_ptr.as_ref().next;
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
            next: GcPtr(self.first_object.0),
            marked: false,
            obj_type,
        });
        let object = GcPtr(NonNull::new(&mut *box_object));
        self.first_object = object;

        self.num_objects += 1;
        object
    }

    pub fn push_int(&mut self, val: i64) {
        let obj = self.new_object(ObjectType::Int(val));
        self.stack.push(obj);
    }

    pub fn push_pair(&mut self) {
        let tail = self.stack.pop().expect("Stack underflow!");
        let head = self.stack.pop().expect("Stack underflow!");
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
