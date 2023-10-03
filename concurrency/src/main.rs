use std::{thread, time::Duration, sync::{mpsc, Mutex, Arc}, rc::Rc};

// Main thread
fn main() {

    /* let handle = thread::spawn(|| {
        // Running in child thread
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Running in parent thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // The spawned thread stops after the main program is done.

    handle.join().unwrap(); // This joins the child thread to the parent thread after the main thread is done.
    //This allows for the code in child to finish running. */

    /* let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // move gives ownership of variables outside the thread to the thread.
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();


    // Message passing through channels
    // mpsc: multi procuder, single consumer.
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone(); // Get another handler to send messages from a different thread

    thread::spawn(move || {
        /* let msg = String::from("Hi");
        thread::sleep(Duration::from_secs(3));
        tx.send(msg).unwrap(); // The recieving thread takes ownership of msg */

        let vals = vec![
            String::from("hi"),
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
        println!("Got: {}", received);
    } */

    /* let recieved = rx.recv().unwrap();
    println!("Got {}!", recieved); */

    // Sharing state
    // Mutex - mutual exclusion
    // A thread locks data in a mutex and unlocks it after use for other threads to have access by creating a lock on the data.

    /* let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // Aquire a lock on the data in the mutex. A lock must not exist on the data if not it fails
        // Deref of num gives acess to the actual data
        // When the mutex goes out of scope, /it releases the lock to the data

        *num = 6;
    }

    println!("{:?}", m); */


    // Sharing data across 10 different threads
    let counter = Arc::new(Mutex::new(0)); // Arc is just like Rc but can be shared between threads.
    let mut hanldes = vec![];

    for i in 1..10 {
        let counter = Arc::clone(&counter);
        hanldes.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }))
    }

    for handle in hanldes {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
