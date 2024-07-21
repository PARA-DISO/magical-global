/**
 * Example Variable Sharing.
 * If this code works correctly,the flag is changed
 * 5000ms after the main thread and the thread is released
 * from the infinite loop.
 *
 */
use magical_global as magical;
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
    let data = TestData::new();
    // set data to magical memory...
    if magical::set_at(Box::new(data), 0).is_err() {
        println!("failed to set data");
        return;
    };

    // generate test thread
    let thread_join_handle = thread::spawn(thread_func);
    thread::sleep(time::Duration::from_millis(5000));

    println!("flag set start @ main");
    let data = magical::get_at_mut::<TestData>(0).unwrap();
    // Change data in mysterious memory
    data.flag = 0;
    println!("flag set end @ main");
    let res = thread_join_handle.join().unwrap();
    // Check to see if the thread is closed
    // If the thread is not closed, it will not be executed.
    println!("Res: {}", res);
}
fn thread_func() -> usize {
    println!("start loop @ Thread");
    loop {
        thread::sleep(time::Duration::from_millis(100));
        let data = magical::get_at::<TestData>(0).unwrap();

        if data.flag == 0 {
            break;
        }
    }
    println!("brake loop! @ Thread");
    return 0;
}
