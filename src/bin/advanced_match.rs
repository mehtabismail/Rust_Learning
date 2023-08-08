enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Bolly".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(70.0, "Amy".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage Ticket holder: {:?}, price: {:?}", holder, price)
            }
            Ticket::Standard(price) => {
                println!("Backstage Ticket price: {:?}", price)
            }
            Ticket::Vip(price, holder) => {
                println!("Vip Ticket holder: {:?}, {:?}", holder, price)
            }
        }
    }
}
