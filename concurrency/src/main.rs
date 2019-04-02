fn main() {
    thread_demo();
    move_ownership_demo();
    message_passing_demo();
    // message_passing_iter_demo();
    // message_passing_multiple_producer_demo();
    mutex_demo();
    arc_demo();
}

fn thread_demo() {
    use std::thread;
    use std::time::Duration;

    main();

    fn main() {
        let handler = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });


        // uncomment, affact the main thread run until the spawned thread terminates
        // handler.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // join block the thread currently running until spawn thread terminates
        handler.join().unwrap();
    }
}

fn move_ownership_demo() {
    use std::thread;

    main();

    fn main() {
        let v = vec![1, 2, 3];

        // move force the closure to take ownership of the values it uses
        let handler = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handler.join().unwrap();
    }
}

fn message_passing_demo() {
    // multiple producer, single consumer
    use std::sync::mpsc;
    use std::thread;

    main();

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        // recv, short for receive, block the thread
        // wait until a value is sent down the channel
        let received = rx.recv().unwrap();
        println!("Got: {}", received);

        // rx has try_recv method, which doesn't block
        // return Result<T, E> immediately
    }

}


fn message_passing_iter_demo() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    main();

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("Hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got {}", received);
        }
    }
}

fn message_passing_multiple_producer_demo() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    main();

    fn main() {
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("message"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    
    }
}


fn mutex_demo() {
    use std::sync::Mutex;

    main();

    fn main() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }
}

fn arc_demo() {
    // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations
    // Atomic Reference Counting
    use std::sync::{Mutex, Arc};
    use std::thread;

    main();

    fn main() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handler);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}

// Mutex<T> provides interior mutability, similarly as RefCell<T> does
// we use Mutex<T> to mutate contents inside an Arc<T>
// Mutex<T> comes with the risk of creating deadlocks
