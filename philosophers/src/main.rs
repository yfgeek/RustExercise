use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
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
        let _left = table.forks[self.left].lock().unwrap();
        println!("哲♂学家 {} 拿起左边的叉子",self.name);
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();
        println!("哲♂学家 {} 拿起右边的叉子",self.name);

        println!("哲♂学家 {} 正在吃", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("哲♂学家 {} 吃完了", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("小王", 0, 1),
        Philosopher::new("小李", 1, 2),
        Philosopher::new("小张", 2, 3),
        Philosopher::new("小刘", 3, 4),
        Philosopher::new("小吴", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}