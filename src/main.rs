//https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html
use std::sync::{Arc, Mutex}; //Arc == Atomic reference count
use std::thread;
use std::time;

struct Philosopher {
    name: String,
    left: usize, // usize is the size that indexes Vectors
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _ = table.forks[self.left].lock().unwrap();
        let _ = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        let d = time::Duration::from_millis(1000);
        thread::sleep(d);

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn f1() {
    let x = 5 + /* 90 + */ 5;

    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_int = 5i32;

    println!("x = {}", x);
    println!("logical:{} flt:{} int:{}", logical, a_float, an_int);

    let _def_flt = 3.0; // f64
    let _def_int = 7; // i32

    let mut inferred_type = 12;
    println!("inf_type:{}", inferred_type);
    inferred_type = 4294967296i64;
    println!("inf_type:{}", inferred_type);

    let inferred_type = true;
    println!("inf_type:{}", inferred_type);

    println!("one million :{}", 1_000_000u32);
}

fn cl1() {
    let color = "green";
    let print = || println!("`color`:{}", color);

    print();
    print();
}

fn main() {
    f1();
    cl1();

    println!("-----------");
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
