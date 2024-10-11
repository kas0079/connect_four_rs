use crate::board;
#[test]
fn diag() {
    let mut b = board::Board::new();
    b.put_debug(7, 0);
    b.put_debug(6, 1);
    b.put_debug(5, 2);
    b.put_debug(4, 3);
    assert_eq!(b.winner(), Some(board::Coin::Red));
}
