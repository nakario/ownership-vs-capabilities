use std::thread;

fn copiable() {
  let mut a: i32 = 1;

  // thread::spawn(|| { // #1
  thread::spawn(move || {
    a += 1;
    println!("another thread: {}", a);
  }).join().unwrap();

  a *= 3;

  println!("main thread: {}", a);
}

fn uncopiable() {
  let mut a: A = A{b: B{c: "c"}};

  thread::spawn(move || {
    // a.b = &mut B{c: "d"}; // #2
    println!("another thread: {}", a.b.c);
  }).join().unwrap();

  // println!("main thread: {}", a.b.c); // #3
}

fn reference() {
  let b: &mut B = &mut B{c: "c"};
  let a: Aref = Aref{b: b};

  thread::spawn(move || {
    // println!("another thread: {}", a.b.c); // #4
  }).join().unwrap();

  println!("main thread: {}", b.c);
}

struct A<'a> {
  b: B<'a>,
}

struct Aref<'a> {
  b: &'a B<'a>,
}

struct B<'a> {
  c: &'a str,
}

fn main() {
  copiable();
  uncopiable();
  reference();
}
