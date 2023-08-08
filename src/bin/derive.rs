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
    // match me.position {
    //     Position::Manager => println!("I'm a manager"),
    //     Position::Supervisor => println!("I' am a supervisor "),
    //     Position::Worker=> println!("I am Worker")
    // }
    println!("{:?}", me.position)
}
