mod board;

fn main() {
    let mut b = board::Board::new();
    b.place(0).expect("uwu");
    b.place(0).expect("uwu");
    b.place(1).expect("uwu");
    b.place(0).expect("uwu");
    b.place(2).expect("uwu");
    b.place(0).expect("uwu");
    b.place(3).expect("uwu");
    println!("{b}");
    println!("winner {:?}", b.winner())
}
