use std::thread;

fn main() {
  let t1 = thread::spawn(func);
  println!("Hello from main!");
  t1.join().unwrap();

  let numbers = Vec::from_iter(0..=500);
  let t2 = thread::spawn(move || {
    let len = numbers.len();
    let sum = numbers.into_iter().sum::<usize>();
    sum / len
  });
  let avg = t2.join().unwrap();
  println!("Avg is {avg}");
}

fn func() {
  let id = thread::current().id();
  println!("This is another thread with id {id:?}");
}
