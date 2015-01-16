#![allow(unstable)]

extern crate "recursive_sync" as rs;

use rs::RMutex;
use std::thread::Thread;
use std::sync::Arc;

#[test]
fn recursive_test() {
    let mutex = RMutex::new(0i32);
    {
        let mut outer_lock = mutex.lock();
        {
            let mut inner_lock = mutex.lock();
            *inner_lock = 1;
        }
        *outer_lock = 2;
    }
    assert_eq!(*mutex.lock(), 2);
}

#[test]
fn test_guarding() {
    let count = 10000;
    let mutex = Arc::new(RMutex::new(0i32));
    let mut guards = Vec::new();

    for _ in (0..count) {
        let mutex = mutex.clone();
        guards.push(Thread::scoped(move || {
            *mutex.lock() += 1;
        }));
    }

    drop(guards);

    assert_eq!(*mutex.lock(), count);
}
