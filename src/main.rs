use std::io;
use std::io::Write;
use std::collections::HashMap;
use rand::Rng;

const DICE_COUNT: usize = 3;

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Die {
    Fish,
    Gourd,
    Tiger,
    Crab,
    Prawn,
    Cock,
}


fn roll_die() -> Die {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1..=6);
    match n {
        1 => Die::Fish,
        2 => Die::Gourd,
        3 => Die::Tiger,
        4 => Die::Crab,
        5 => Die::Prawn,
        _ => Die::Cock,
    }
}

fn roll(dice: &mut [Die; 3]) {
    for i in 0..DICE_COUNT {
        dice[i] = roll_die();
    }
}

fn reset_bets(bets: &mut HashMap<Die, i32>) {
    bets.insert(Die::Fish, 0);
    bets.insert(Die::Gourd, 0);
    bets.insert(Die::Tiger, 0);
    bets.insert(Die::Crab, 0);
    bets.insert(Die::Prawn, 0);
    bets.insert(Die::Cock, 0);
}

fn main() {
    let bet_order = [Die::Fish, Die::Gourd, Die::Tiger, Die::Crab, Die::Prawn, Die::Cock];

    let mut dice = [Die::Fish; DICE_COUNT];

    let mut bets: HashMap<Die, i32> = HashMap::new();

    print!("Initial money: ");
    io::stdout().flush().unwrap();

    let mut money = String::new();
    io::stdin()
        .read_line(&mut money)
        .expect("Failed to read line");
    let mut money: i32 = match money.trim().parse() {
        Ok(num) => num,
        Err(_) => 100,
    };

    loop {
        // Print stats
        println!("Current money: {money}");

        // handle bets
        reset_bets(&mut bets);
        let mut total_bet: i32 = 0;
        for face in bet_order {
            loop {
                print!("Bet on {:?}: ", face);
                io::stdout().flush().unwrap();

                let mut bet_str = String::new();
                io::stdin()
                    .read_line(&mut bet_str)
                    .expect("Failed to read line");

                let bet: i32 = match bet_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0,
                };

                if total_bet + bet <= money {
                    total_bet += bet;
                    bets.insert(face, bet);
                    break;
                } else {
                    println!("You do not have enough money for this bet!");
                    println!("You can only bet up to: {}\n", money - total_bet);
                }
            }
            if total_bet == money {
                break;
            }
        }

        // roll
        let mut win = 0;
        roll(&mut dice);
        println!("");
        for i in 0..DICE_COUNT {
            let bet = bets.get(&dice[i]).unwrap();
            win += bet;
            print!("[{:?}: {}] ", dice[i], bet);
        }
        println!("\n");

        if win > 0 {
            println!("You won {}!", win);
            money += win;
        }
        else {
            println!("You lost {}", -total_bet);
            money -= total_bet;
        }

        // end condition check
        if money == 0 {
            println!("You ran out of money!");
            break;
        }
    }
}
