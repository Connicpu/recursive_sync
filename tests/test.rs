extern crate "recursive_sync" as rs;

use rs::RMutex;

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
