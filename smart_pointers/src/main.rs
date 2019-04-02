fn main() {
    box_pointer();
    my_box();
    drop_trait();

    rc_generic_demo();
    refcell_generic_demo();
    reference_cycles_demo();
    weak_reference_demo();
}

// Box<T>, allow you to store data on the heap rather than stack.
// no performance overhead, no extra capabilities.
// 1. When a type whose size can't be known at compile time,
//    and you want to use it in a context requires an exact size.
// 2. When a large amount of data, and you want to transfer the ownership,
//    but ensure the data won't be copied when you do so.
// 3. When you want to own a value and you care only that it's a type
//    that implements a pariticular triat rather than being of a specific type.
fn box_pointer() {
    // Box<T> implements Deref trait, allows Box<T> values to be treated like references.
    let b = Box::new(5);
    println!("b = {}", b);


    use List::{Cons, Nil};


    // using Box let compiler know this enum spaces
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let _list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    // Box<T> type implements Drop trait, the head data will be cleaned up
    // when Box<T> value goes out of scope
}


fn my_box() {
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // impl Deref trait to let MyBox<T> values dereference
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        // define associated type for Deref trait to use
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }


    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // *y actually equals to *(y.deref()), deref method returns a normal reference
    assert_eq!(5, *y);


    /*
     * implicit deref coercions with functions and methods
     */
    fn coercions() {
        let m = MyBox::new(String::from("Rust"));
        // because we impl Deref trait on MyBox<T> type.
        // MyBox<T> is implicit convert to &String
        // String also implict Deref that returns a string slice.
        // &String is turn into &str type, eventually.
        hello(&m);
        // If no coercions in Rust, we need write it in this way.
        // hello(&(*m)[..]);

        fn hello(name: &str) {
            println!("Hello {}!", name);
        }
    }

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    // 1. From &T to &U when T: Deref<Target=U>
    // 2. From &mut T to &mut U when T: DerefMut<Target=U>
    // 3. From &mut T to &U when T:Deref<Target=U>
}

fn drop_trait() {

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data {}", self.data);
        }
    }

    drop_demo();

    fn drop_demo() {
        use std::mem::drop;

        let c = CustomSmartPointer { data: String::from("my stuff") };
        let d = CustomSmartPointer { data: String::from("other stuff") };
        println!("CustomSmartPointers created.");
        // force early drop
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}

/**
 * Rc<T> gives the reference count version of Box<T>
 * immutable reference. no races and inconsistences
 */
fn rc_generic_demo() {
    use List::{Cons, Nil};
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    main();

    fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons (10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        // 3 -> 5 -> 10 -> Nil
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            // 4 -> 5 -> 10 -> Nil
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));

        // Rc::clone just increase reference instead of deep copy
    }

}


/**
 * RefCell<T> represents single ownership
 * RefCell<T> enforce the borrowing rules at runtime,
 * if broken, program will panic and exit.
 */
fn refcell_generic_demo() {
    use List::{Cons, Nil};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    main();

    fn main() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

/**
 * recap:
 * - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
 * - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only
 *   immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows
 *   checked at runtime.
 * - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside
 *   the RefCell<T> even when the RefCell<T> is immutable.
 */
fn _recap() {}


fn reference_cycles_demo() {
    use std::rc::Rc;
    use std::cell::RefCell;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    main();

    fn main() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // // it will overflow the stack
        // println!("a next item  = {}", a.tail());
    }
}

fn weak_reference_demo() {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    main();

    fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("branch strong = {}, weak = {}",
                     Rc::strong_count(&branch),
                     Rc::weak_count(&branch),
            );
            println!("leaf strong = {}, weak = {}",
                     Rc::strong_count(&leaf),
                     Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf),
        );
    }
}
