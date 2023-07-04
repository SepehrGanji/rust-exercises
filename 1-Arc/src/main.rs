use std::sync::Arc;
use std::thread;

fn main() {
    println!("Hello, world!");
    let nums = Arc::new([1, 2, 3, 4]);

    let t = thread::spawn({
      let nums = nums.clone();
      move || {
        dbg!(nums);
      }
    });

    dbg!(nums);

    t.join().unwrap();
}
