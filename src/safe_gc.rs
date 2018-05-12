const INITIAL_GC_THRESHOLD: usize = 32;
const DEFAULT_HEAP_SIZE: usize = 1 << 16;

#[derive(Clone, Debug)]
struct GcPtr(usize);

impl GcPtr {
    fn mark(&self, heap: &mut Vec<Option<Object>>) {
        let mut object = heap[self.0].clone().unwrap();

        if object.marked {
            return;
        }

        object.marked = true;
        heap[self.0] = Some(object.clone());

        if let ObjectType::Pair(pair) = object.obj_type {
            if let Some(head) = pair.head {
                head.mark(heap);
            }

            if let Some(tail) = pair.tail {
                tail.mark(heap);
            }
        }
    }
}

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
    head: Option<GcPtr>,
    tail: Option<GcPtr>,
}

pub struct Vm {
    num_objects: usize,
    max_objects: usize,
    heap: Vec<Option<Object>>,
    free_list: Vec<GcPtr>,
    stack: Vec<GcPtr>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            num_objects: 0,
            max_objects: INITIAL_GC_THRESHOLD,
            heap: Vec::with_capacity(DEFAULT_HEAP_SIZE),
            free_list: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn with_capacity(size: usize) -> Vm {
        Vm {
            num_objects: 0,
            max_objects: INITIAL_GC_THRESHOLD,
            heap: Vec::with_capacity(size),
            free_list: Vec::new(),
            stack: Vec::new(),
        }
    }

    fn mark_all(&mut self) {
        for object in self.stack.iter() {
            object.mark(&mut self.heap);
        }
    }

    fn sweep(&mut self) {
        for (i, object) in self.heap
            .iter_mut()
            .enumerate()
            .filter(|elem| elem.1.is_some())
        {
            match object.clone() {
                Some(obj) => if !obj.marked {
                    self.num_objects -= 1;

                    *object = None;
                    self.free_list.push(GcPtr(i));
                } else {
                    object.as_mut().unwrap().marked = false;
                },
                None => unreachable!(),
            }
        }
    }

    pub fn gc(&mut self) {
        let num_objects = self.num_objects;

        self.mark_all();
        self.sweep();

        self.max_objects = num_objects * 2;
    }

    fn new_object(&mut self, obj_type: ObjectType) -> GcPtr {
        if self.num_objects == self.max_objects {
            self.gc();
        }

        self.num_objects += 1;

        let object = Object {
            marked: false,
            obj_type,
        };

        match self.free_list.pop() {
            Some(val) => {
                self.heap[val.0] = Some(object);
                val
            }
            None => {
                self.heap.push(Some(object));
                GcPtr(self.heap.len() - 1)
            }
        }
    }

    pub fn push_int(&mut self, val: i64) {
        let obj = self.new_object(ObjectType::Int(val));
        self.stack.push(obj);
    }

    pub fn push_pair(&mut self) {
        let tail = Some(self.stack.pop().expect("Stack underflow!"));
        let head = Some(self.stack.pop().expect("Stack underflow!"));
        let obj = self.new_object(ObjectType::Pair(Pair { head, tail }));
        self.stack.push(obj);
    }

    pub fn drop(&mut self) {
        self.stack.pop();
    }
}

// I think this is unnecessary, actually
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
    fn test_safe_gc() {
        let mut vm = Vm::new();

        for i in 0..64 {
            vm.push_int(i);
        }
        for _ in 0..63 {
            vm.push_pair();
        }

        assert_eq!(vm.stack.len(), 1);
        assert_ne!(vm.num_objects, 0);
        assert_ne!(vm.heap.len(), 0);

        vm.stack.pop();

        vm.gc();

        assert_eq!(vm.num_objects, 0);
        assert!(vm.heap.iter().all(|e| e.is_none()))
    }
}
