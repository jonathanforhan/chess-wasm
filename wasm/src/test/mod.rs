mod perft;
#[cfg(test)]
mod test {
use super::super::game::{
    fen,
    pieces::{Color::*, *},
};

/* cargo test [TEST NAME] -- --nocapture */
#[test]
fn test_fen() {
    let fen = "r3k3/p1pNqpbr/bn2Pnp1/8/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 2";
    let game = fen::decode(fen).unwrap();
    let fen_encoded = fen::encode(&game).unwrap();

    assert_eq!(fen, fen_encoded);
}

#[test]
fn test_move() {
    let game = fen::decode("4k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/P2P2PP/r2Q1R1K w k - 0 2").unwrap();
    let (mut w, mut b) = (0, 0);
    for p in &game.pieces {
        match p.color() {
            White => w |= p.bits(),
            Black => b |= p.bits(),
        }
    }

    print_bits(&(w|b), 'o');

    let mv = game.moves();
    for m in &mv {
        if let Pieces::Pawn(r) = m {
            print_bits(&r.bits(), 'r');
        }
    }

    println!("{}", mv.len());
}

pub fn print_bits(x: &u128, c: char) {
    for i in (-15..=112).rev().step_by(16) { // 0..128 but with rev-step
        for j in 0..8 {
            print!("{} ", (x >> (i + j + 8) & 1)
                   .to_string()
                   .replace('1', &c.to_string())
                   .replace('0', "."));
        }
        //println!("* * * * * * * *");
        print!("\n");
    }
    print!("\n");
}
}
