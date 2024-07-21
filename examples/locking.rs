/**
 * Example Variable Sharing with Mutex lock.
 * If this code works correctly, it will loop infinitely.
 *
 */
use magical_global as magical;
use std::sync::Mutex;
use std::{thread, time};
struct TestData {
    flag: usize,
}
impl TestData {
    fn new() -> Self {
        Self { flag: 1 }
    }
}
fn main() {
    let data = Mutex::new(TestData::new());
    // set data to magical memory...
    if magical::set_at(Box::new(data), 0).is_err() {
        println!("failed to set data");
        return;
    };

    // generate test thread
    let thread_join_handle = thread::spawn(thread_func);
    thread::sleep(time::Duration::from_millis(5000));

    println!("flag set start @ main");
    let data = magical::get_at_mut::<Mutex<TestData>>(0).unwrap();
    // Change data in magical memory
    {
        // get data locked by thread
        // Thread must be unlocked to proceed.
        let mut tmp = data.lock().unwrap();
        tmp.flag = 0;
    }
    println!("flag set end @ main");
    let res = thread_join_handle.join().unwrap();
    // Check to see if the thread is closed
    // If the thread is not closed, it will not be executed.
    println!("Res: {}", res);
        // take ownership of the sharing data
    let _ = magical::take_at::<Mutex<TestData>>(0);
}
fn thread_func() -> usize {
    println!("start loop @ Thread");
    let data = magical::get_at::<Mutex<TestData>>(0).unwrap();
    // locking this data.
    let tmp = data.lock().unwrap();
    loop {
        thread::sleep(time::Duration::from_millis(100));
        if tmp.flag == 0 {
            break;
        }
    }
    println!("brake loop! @ Thread");
    return 0;
}
