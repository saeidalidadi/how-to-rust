enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64)
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(34.65, "Saeid".to_owned()),
        Ticket::Standard(23.44),
        Ticket::Vip(100.23, "John".to_owned())
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Vip(price, holder) => println!("Holder: {:?} Price: {:?}", holder, price),
            Ticket::Backstage(price, holder) => println!("Holder: {:?} Price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Price: {:?}", price),
        } 
    }
}
