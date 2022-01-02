
#[derive(Copy, Clone, Debug)]
struct Losses {
    attacker: u32,
    defender: u32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dice {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6
}

impl Dice {
    fn all() -> Vec<Dice> {
        use Dice::*;
        vec![ONE, TWO, THREE, FOUR, FIVE, SIX]
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Strategy {
    WithThree,
    WithTwo,
    WithOne
}

#[derive(Copy, Clone, Debug)]
enum Attack {
    WithThree(Dice, Dice, Dice),
    WithTwo(Dice, Dice),
    WithOne(Dice)
}

impl Attack {
    fn strategy(&self) -> Strategy {
        match self {
            &Self::WithOne(_) => Strategy::WithOne,
            &Self::WithTwo(_, _) => Strategy::WithTwo,
            &Self::WithThree(_, _, _) => Strategy::WithThree
        }
    }

    fn all() -> Vec<Attack> {
        use Attack::*;
        let mut variations = vec![];        
        for first_dice in Dice::all().iter() {
            variations.push(WithOne(*first_dice));
            for second_dice in Dice::all().iter() {
                if first_dice >= second_dice {
                    variations.push(WithTwo(*first_dice, *second_dice));
                }
                else {
                    variations.push(WithTwo(*second_dice, *first_dice));
                }
                for third_dice in Dice::all().iter() {
                    let mut all = vec![*first_dice, *second_dice, *third_dice];
                    all.sort();
                    all.reverse();
                    variations.push(WithThree(all[0], all[1], all[2]));
                }
            }    
        }
        variations
    }
}

#[derive(Copy, Clone, Debug)]
enum Defend {
    WithTwo(Dice, Dice),
    WithOne(Dice)
}

impl Defend {
    fn strategy(&self) -> Strategy {
        match self {
            &Self::WithOne(_) => Strategy::WithOne,
            &Self::WithTwo(_, _) => Strategy::WithTwo
        }
    }

    fn all() -> Vec<Defend> {
        use Defend::*;
        let mut variations = vec![];        
        for first_dice in Dice::all().iter() {
            variations.push(WithOne(*first_dice));
            for second_dice in Dice::all().iter() {
                if first_dice >= second_dice {
                    variations.push(WithTwo(*first_dice, *second_dice));
                }
                else {
                    variations.push(WithTwo(*second_dice, *first_dice));
                }
            }    
        }
        variations
    }
}

fn decide_dice(defend: Dice, attack: Dice) -> Losses {
    if defend >= attack {
        Losses {
            defender: 0,
            attacker: 1
        }
    }
    else {
        Losses {
            defender: 1,
            attacker: 0
        }
    }
}

fn decide(attack: Attack, defend: Defend) -> Losses {
    match defend {
        Defend::WithOne(defend_highest) => match attack {
            Attack::WithOne(attack_highest) | Attack::WithTwo(attack_highest, _) | Attack::WithThree(attack_highest, _, _) => {
                decide_dice(defend_highest, attack_highest)
            }
        },
        Defend::WithTwo(defend_highest, defend_second_highest) => match attack {
            Attack::WithOne(attack_highest) => {
                decide_dice(defend_highest, attack_highest)
            },
            Attack::WithTwo(attack_highest, attack_second_highest) | Attack::WithThree(attack_highest, attack_second_highest, _) => {
                decide_dice(defend_highest, attack_highest) + decide_dice(defend_second_highest, attack_second_highest)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct StrategySummary {
    attack: Strategy,
    defend: Strategy
}

impl StrategySummary {
    fn new(attack: Attack, defend: Defend) -> StrategySummary {
        StrategySummary {
            attack: attack.strategy(),
            defend: defend.strategy()
        }
    }

    fn from_strategy(attack: Strategy, defend: Strategy) -> StrategySummary {
        StrategySummary {
            attack,
            defend
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct LossesSummary {
    losses: Losses,
    occurrences: u32
}

impl LossesSummary {
    fn new(losses: Losses) -> LossesSummary {
        LossesSummary {
            losses,
            occurrences: 1
        }
    }
}

use std::ops::Add;

impl Add for Losses {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            defender: self.defender + other.defender,
            attacker: self.attacker + other.attacker
        }
    }
}

impl Add<Losses> for LossesSummary {
    type Output = Self;

    fn add(self, other: Losses) -> Self {
        Self {
            losses: self.losses + other,
            occurrences: self.occurrences + 1
        }
    }
}

fn average_losses(losses: LossesSummary) -> String {
    format!("A: {:0.2}, D: {:0.2}", 
        (losses.losses.attacker as f32) / (losses.occurrences as f32),
        (losses.losses.defender as f32) / (losses.occurrences as f32))
}

fn main() {
    use std::collections::HashMap;

    let mut strategy_summaries  = HashMap::new();

    for attack in Attack::all().iter() {
        for defend in Defend::all().iter() {
            let outcome = decide(*attack, *defend);
            println!("{:?},{:?} -> {:?}", attack, defend, outcome);
            let key = StrategySummary::new(*attack, *defend);
            match strategy_summaries.get(&key) {
                Some(summary) => {
                    strategy_summaries.insert(key, *summary + outcome);
                },
                None => {
                    strategy_summaries.insert(key, LossesSummary::new(outcome));
                }
            }
        }
    }
    
    for (summary, losses) in strategy_summaries.iter() {
        let averages = format!("A: {:0.2}, D: {:0.2}", 
            (losses.losses.attacker as f32) / (losses.occurrences as f32),
            (losses.losses.defender as f32) / (losses.occurrences as f32));
        println!("A: {:?}, D: {:?}, average losses: {}, losses: {:?}", summary.attack, summary.defend, averages, losses);
    }

    println!(
        "{: <15}| {: <20} | {: <20} | {: <20}",
        "", "attack: 3", "attack: 2", "attack: 1"
    );
    for (defend_strategy, title) in vec![(Strategy::WithTwo, "defend: 2"), (Strategy::WithOne, "defend: 1")].iter() {
        let attack_with_three_summary 
            = strategy_summaries.get(&StrategySummary::from_strategy(Strategy::WithThree, *defend_strategy)).unwrap();
        let attack_with_two_summary 
            = strategy_summaries.get(&StrategySummary::from_strategy(Strategy::WithTwo, *defend_strategy)).unwrap();
        let attack_with_one_summary 
            = strategy_summaries.get(&StrategySummary::from_strategy(Strategy::WithOne, *defend_strategy)).unwrap();
        println!(
            "{: <15}| occurrences: {: <7} | occurrences: {: <7} | occurrences: {: <7}",
            title, 
            attack_with_three_summary.occurrences,
            attack_with_two_summary.occurrences,
            attack_with_one_summary.occurrences,
        );
        println!(
            "{: >15}| {: <20} | {: <20} | {: <20}",
            "avg loss: ", 
            average_losses(*attack_with_three_summary),
            average_losses(*attack_with_two_summary),
            average_losses(*attack_with_one_summary),
        );
    }
}
