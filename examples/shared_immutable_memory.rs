use std::sync::Arc;

#[deriving(Show)]
struct HugeStruct {
    huge_name: String
}

impl HugeStruct {
    fn new() -> HugeStruct {
        HugeStruct { huge_name: "I'M HUGE".to_string() }
    }
}

fn main() {
    let (tx, rx) = channel();
    let huge_struct = Arc::new(HugeStruct::new());

    for task_num in range(0u, 10) {
        let tx = tx.clone();
        let huge_struct = huge_struct.clone();
        spawn(proc() {
            let msg = format!("Task {}: Accessed {}", task_num, huge_struct.huge_name);
            tx.send(msg);
        });
    }
    drop(tx); // => force last transmitter to hang up

    for data in rx.iter() {
        println!("{}", data);
    }
}
