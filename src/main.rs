#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

struct Employee {
    position: Position,
    work_hours: i64,
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    println!("{:?}", me.position)
}
