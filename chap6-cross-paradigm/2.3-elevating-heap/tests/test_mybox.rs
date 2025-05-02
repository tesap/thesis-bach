
#[derive(Debug)]
struct Obj(i32);

impl Drop for Obj {
    fn drop(&mut self) {
        println!("DROP {}", self.0);
        self.0 = -1;
    }
}

impl Clone for Obj {
    fn clone(&self) -> Self {
        println!("CLONE");
        Self { 0: self.0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::Obj;
    use elevating_heap::MyBox;
    use object_on_heap::object_alloc_init;

    fn new() -> MyBox<Obj> {
        let b: MyBox<Obj> = MyBox::new(
            Obj { 0: 1 }
        );
        b
    }

    #[test]
    fn test_new() {
        new();
    }

    #[test]
    fn test_deref() {
        let b = new();
        // Direct Deref trait
        println!("Deref: {:?}", *b);
    }

    #[test]
    fn test_clone() {
        let b = new();

        // Clone via "deref coercions"
        let a = b.clone();
    }

    #[test]
    fn test_clone_from() {
        let b = new();
        let mut b2 = new();

        println!("==");
        // Does not call clone_from for some reason
        // b2 = b.clone();
        b2.clone_from(&b);
        println!("==");
    }

    #[test]
    fn test_into_raw() {
        let b = new();

        let p: *mut Obj = b.into_raw();

        // === Should fail to compile
        // unsafe {
        //     println!("{:?}", b);
        // }
    }

    #[test]
    fn test_from_raw() {
        let p: *mut Obj = object_alloc_init(Obj { 0: 23 });
        let b = MyBox::from_raw(p);

        // The two-fold nature of pointer serving as both referencing and ownership semantics
        // leads to the fact that it does not have strict ownership semantics
        println!("{:?}", p);
    }
}

