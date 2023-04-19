use std::thread;

fn thread_worker(name: &str) {
    println!("Привет {}! Из потока {:?}", name, thread::current().id());
}

fn main() {
    let handle1 = thread::spawn(move || thread_worker("А"));

    let handle2 = thread::spawn(move || thread_worker("Б"));

    handle1.join().unwrap();
    handle2.join().unwrap();
}
