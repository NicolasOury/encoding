use encoding::Encoding;
/*** An example of simple game encoding: 3 cards limit Poker
 * There is 3 cards A > K > Q
 * There is at most 3 betting.
 */
#[derive(Encoding, Debug)]
enum Card { A, K, Q}
#[derive(Debug)]
struct Game {
    hole_card1 : Card,
    hole_card2 : Card,
    history: [Option<Move>; 3]
}

#[derive(Encoding, Debug)]
enum Move { 
    Call,
    Raise,
    Fold
}

#[derive(Encoding, Debug)]
struct PlayerView {
    hole_card: Card,
    history: [Option<Move>; 3]
}

fn legal_moves(pv : &PlayerView) -> Vec<Move> {
    let mut v = vec![Move::Call, Move::Fold];
    match pv.history[2] {
        None => v.push(Move::Raise),
        Some(_) => ()
    };
    v
}

fn main() {
    println!("{:?}", PlayerView{hole_card: Card::A, history:[Some(Move::Call), Some(Move::Raise),None]}.encode());
    let v = [0.7, 0.8, 0.2];
    println!("C {:?}, R {:?}, F {:?}", Move::Call.likelihood(&v), Move::Raise.likelihood(&v), Move::Fold.likelihood(&v));
}
