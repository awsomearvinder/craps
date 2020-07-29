use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
fn roll2die<T: Rng>(mut rng: T) -> (i32, i32) {
    let first_roll = rng.gen_range(1, 7);
    let second_roll = rng.gen_range(1, 7);
    (first_roll, second_roll)
}
fn main() {
    let mut thread_rng = SmallRng::seed_from_u64(1);
    let mut string = String::with_capacity(10_000_000);
    for _ in 0..1_000_000 {
        run(&mut thread_rng, &mut string);
    }
    print!("{}", string);
}
fn run<T: Rng>(mut thread_rng: T, buffer: &mut String) {
    let mut pt = match game(&mut thread_rng, &[7, 11], &[2, 3, 12]) {
        GameStates::Win => {
            buffer.push_str("you win! \n");
            return;
        }
        GameStates::Lose => {
            buffer.push_str("you lose!\n");
            return;
        }
        GameStates::Neither(num) => num,
    };
    loop {
        match game(&mut thread_rng, &[pt], &[7]) {
            GameStates::Win => {
                buffer.push_str("you win!\n");
                break;
            }
            GameStates::Lose => {
                buffer.push_str("you lose!\n");
                break;
            }
            GameStates::Neither(num) => pt = num,
        }
    }
}
enum GameStates {
    Win,
    Lose,
    Neither(i32),
}
fn game<T: Rng>(rng: T, win: &[i32], lose: &[i32]) -> GameStates {
    let (first_roll, second_roll) = roll2die(rng);
    if win.iter().any(|&num| num == first_roll + second_roll) {
        GameStates::Win
    } else if lose.iter().any(|&num| num == first_roll + second_roll) {
        GameStates::Lose
    } else {
        GameStates::Neither(first_roll + second_roll)
    }
}
