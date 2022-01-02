#[derive(Copy, Clone, Debug)]
struct Losses {
    defender: u8,
    attacker: u8
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

#[derive(Copy, Clone, Debug)]
enum Attack {
    WITH_THREE(Dice, Dice, Dice),
    WITH_TWO(Dice, Dice),
    WITH_ONE(Dice)
}

impl Attack {
    fn all() -> Vec<Attack> {
        use Attack::*;
        let mut variations = vec![];        
        for first_dice in Dice::all().iter() {
            variations.push(WITH_ONE(*first_dice));
            for second_dice in Dice::all().iter() {
                if first_dice >= second_dice {
                    variations.push(WITH_TWO(*first_dice, *second_dice));
                }
                else {
                    variations.push(WITH_TWO(*second_dice, *first_dice));
                }
                for third_dice in Dice::all().iter() {
                    let mut all = vec![*first_dice, *second_dice, *third_dice];
                    all.sort();
                    variations.push(WITH_THREE(all[0], all[1], all[2]));
                }
            }    
        }
        variations
    }
}

#[derive(Copy, Clone, Debug)]
enum Defend {
    WITH_TWO(Dice, Dice),
    WITH_ONE(Dice)
}

impl Defend {
    fn all() -> Vec<Defend> {
        use Defend::*;
        let mut variations = vec![];        
        for first_dice in Dice::all().iter() {
            variations.push(WITH_ONE(*first_dice));
            for second_dice in Dice::all().iter() {
                if first_dice >= second_dice {
                    variations.push(WITH_TWO(*first_dice, *second_dice));
                }
                else {
                    variations.push(WITH_TWO(*second_dice, *first_dice));
                }
            }    
        }
        variations
    }
}

fn decide(attack: Attack, defend: Defend) -> Losses {
    match defend {
        Defend::WITH_ONE(defend_highest) => match attack {
            Attack::WITH_ONE(attack_highest) => {
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
            Attack::WITH_TWO(attack_highest, _) => {
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
            Attack::WITH_THREE(attack_highest, _, _) => {
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
        Defend::WITH_TWO(defend_highest, defend_second_highest) => match attack {
            Attack::WITH_ONE(attack_highest) => {
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
            Attack::WITH_TWO(attack_highest, attack_second_highest) => {
                let mut attacker : u8 = 0;
                let mut defender : u8 = 0;
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
            Attack::WITH_THREE(attack_highest, attack_second_highest, _) => {
                let mut attacker : u8 = 0;
                let mut defender : u8 = 0;
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

fn main() {
    for attack in Attack::all().iter() {
        for defend in Defend::all().iter() {
            let outcome = decide(*attack, *defend);
            println!("{:?},{:?} -> {:?}", attack, defend, outcome);
        }
    }
}
