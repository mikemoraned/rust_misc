enum Outcome {
    RED_LOSE_TWO,
    BLUE_LOSE_TWO,
    BOTH_LOSE_ONE
}

enum Dice {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6
}

enum Attack {
    WITH_THREE(Dice, Dice, Dice),
    WITH_TWO(Dice, Dice),
    WITH_ONE(Dice)
}

enum Defend {
    WITH_TWO(Dice, Dice),
    WITH_ONE(Dice)
}

fn decide(attack: Attack, defend: Defend) -> Outcome {
    Outcome::BLUE_LOSE_TWO
}

fn main() {
    println!("Hello, world!");
}
