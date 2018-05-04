const INITIAL_GC_THRESHOLD: usize = 32;
const DEFAULT_HEAP_SIZE: usize = 1 << 16;

#[derive(Clone, Copy)]
struct GcPtr(usize);

#[derive(Clone, Copy)]
pub struct Object {
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
            Self::mark(&mut self.heap, *object);
        }
    }

    fn mark(heap: &mut Vec<Option<Object>>, obj_ptr: GcPtr) {
        let mut object = heap[obj_ptr.0].unwrap();

        if object.marked {
            return;
        }

        object.marked = true;

        if let ObjectType::Pair(pair) = object.obj_type {
            if let Some(head) = pair.head {
                Self::mark(heap, head);
            }

            if let Some(tail) = pair.tail {
                Self::mark(heap, tail);
            }
        }
    }

    fn sweep(&mut self) {
        for (i, object) in self.heap
            .iter_mut()
            .enumerate()
            .filter(|elem| elem.1.is_some())
        {
            let mut obj = object.unwrap();
            if !obj.marked {
                *object = None;
                self.free_list.push(GcPtr(i));
            } else {
                obj.marked = false;
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
}

// I think this is unnecessary, actually
impl Drop for Vm {
    fn drop(&mut self) {
        self.stack = vec![];
        self.gc();
    }
}
