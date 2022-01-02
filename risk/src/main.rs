
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

fn decide(attack: Attack, defend: Defend) -> Losses {
    match defend {
        Defend::WithOne(defend_highest) => match attack {
            Attack::WithOne(attack_highest) => {
                if defend_highest >= attack_highest {
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
            },
            Attack::WithTwo(attack_highest, _) => {
                if defend_highest >= attack_highest {
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
            },
            Attack::WithThree(attack_highest, _, _) => {
                if defend_highest >= attack_highest {
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
        },
        Defend::WithTwo(defend_highest, defend_second_highest) => match attack {
            Attack::WithOne(attack_highest) => {
                if defend_highest >= attack_highest {
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
            },
            Attack::WithTwo(attack_highest, attack_second_highest) => {
                let mut attacker = 0;
                let mut defender = 0;
                if defend_highest >= attack_highest {
                    attacker += 1;    
                }
                else {
                    defender += 1;
                }
                if defend_second_highest >= attack_second_highest {
                    attacker += 1;  
                }
                else {
                    defender += 1;
                }
                Losses {
                    attacker,
                    defender
                }
            },
            Attack::WithThree(attack_highest, attack_second_highest, _) => {
                let mut attacker = 0;
                let mut defender = 0;
                if defend_highest >= attack_highest {
                    attacker += 1;    
                }
                else {
                    defender += 1;
                }
                if defend_second_highest >= attack_second_highest {
                    attacker += 1;  
                }
                else {
                    defender += 1;
                }
                Losses {
                    attacker,
                    defender
                }
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
}
