use rand::Rng;

/// Enum for type of die.
#[derive(Clone, Copy, PartialEq)]
pub enum Die {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

/// Struct to represent multiple dice of a single type to be rolled, such as 2d6.
pub struct DiceToRoll {
    pub die: Die,
    pub number: u32,
}

impl DiceToRoll {
    pub const fn new(number: u32, die: Die) -> DiceToRoll {
        return DiceToRoll {
            die: die,
            number: number,
        };
    }

    //TODO: This should really be a const fn,
    // but Rust does not currently support control flow in const fn
    /// Generate a `DiceToRoll` from a string such as '2d6'.
    pub fn from_string(s: String) -> Result<DiceToRoll, String> {
        let mut d = false; // if the d has been declared in the string yet
        let mut num_str: String = "".to_string(); // number of rolls
        let mut die_str: String = "".to_string(); // type of die
        for ch in s.to_lowercase().chars() {
            if ch == 'd' {
                d = true;
                if num_str == "" {
                    num_str = "1".to_string();
                }
            }
            if d {
                die_str.push(ch);
            } else {
                num_str.push(ch);
            }
        }

        let number = match num_str.parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err("Number is not a valid integer".to_string()),
        };

        let die_int = match die_str.parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err("Die is not a valid integer".to_string()),
        };

        let die: Die = match die_int {
            4 => Die::D4,
            6 => Die::D6,
            8 => Die::D8,
            10 => Die::D10,
            12 => Die::D12,
            20 => Die::D20,
            100 => Die::D100,
            _ => return Err("Die is not a valid die type".to_string()),
        };

        return Ok(DiceToRoll::new(number, die));
    }
}

/// Struct to represent multiple rolled dice of a single type.
/// Stores individual rolls and a grand total.
pub struct RolledDice {
    pub die: Die,
    pub rolls: Vec<u32>,
    pub total: u32,
}

/// Struct to represent multiple rolled dice of multiple types.
/// Stores a vec of `RolledDice` and a grand total.
pub struct RolledDiceBatch {
    pub types: Vec<RolledDice>,
    pub total: u32,
}

/// Generate a block of PC stats.
///
/// This is done by calculating each stat by rolling four d6 and dropping the lowest.
/// Returns an array of `u8`.
pub fn gen_stats() -> [u8; 6] {
    let mut stats: [u8; 6] = [0; 6];
    let mut rng = rand::thread_rng();

    for i in 0..6 {
        let mut rolls: [u8; 4] = [0; 4];
        for j in 0..4 {
            rolls[j] = rng.gen_range(1, 7);
        }
        rolls.sort();

        // Skip the first (lowest) roll when adding
        for r in rolls.iter().skip(1) {
            stats[i] += r;
        }
    }
    stats.sort();
    return stats;
}

/// Simulate rolling dice.
///
/// Takes a vec of Rolls as a parameter.
/// Each individual roll is calculated and returned in a RolledDiceBatch.
pub fn roll_dice(rolls: Vec<DiceToRoll>) -> RolledDiceBatch {
    let mut ret = RolledDiceBatch {
        types: Vec::new(),
        total: 0,
    };
    let mut rng = rand::thread_rng();

    // die_type represents each type of die given as a parameter
    for die_type in rolls.iter() {
        // die_rolls is a vector containing each individual roll of the die type (inner vecs of return)
        let mut die_rolls = RolledDice {
            die: die_type.die,
            rolls: Vec::new(),
            total: 0,
        };
        for _ in 0..die_type.number {
            // this_roll is the value of an individual roll
            let this_roll: u32 = rng.gen_range(1, die_type.die as u32 + 1);
            die_rolls.rolls.push(this_roll);
            die_rolls.total += this_roll;
            ret.total += this_roll;
        }
        ret.types.push(die_rolls);
    }

    return ret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats() {
        let s1 = gen_stats();
        let s2 = gen_stats();
        let s3 = gen_stats();
        let s4 = gen_stats();
        let s5 = gen_stats();
        let test_array = [s1, s2, s3, s4, s5];

        for test in test_array.iter() {
            let mut i = 0;
            while i < 6 {
                assert!(test[i] <= 18);
                assert!(test[i] >= 3);
                i += 1;
            }
        }
    }

    #[test]
    fn test_roll_none() {
        let r = roll_dice(Vec::new());
        assert!(r.types.len() == 0);
        assert!(r.total == 0);
    }

    #[test]
    fn test_roll_d20() {
        let d20 = DiceToRoll::new(1, Die::D20);

        let mut v: Vec<DiceToRoll> = Vec::new();
        v.push(d20);

        let r = roll_dice(v);

        assert!(r.types.len() == 1); // one type of die
        assert!(r.types[0].die == Die::D20); // die is d20
        assert!(r.types[0].rolls.len() == 1); // one die roll
        assert!(r.types[0].rolls[0] >= 1); // assert 1 to 20
        assert!(r.types[0].rolls[0] <= 20); // ""
        assert!(r.types[0].rolls[0] == r.total); // one roll = total
    }

    #[test]
    fn test_roll_4d6() {
        let dice = DiceToRoll::new(4, Die::D6);

        let mut v: Vec<DiceToRoll> = Vec::new();
        v.push(dice);

        let r = roll_dice(v);

        assert!(r.types.len() == 1); // one type of die
        assert!(r.types[0].die == Die::D6); // die is D6
        assert!(r.types[0].rolls.len() == 4); // four rolls

        // assert each roll is between 1 and 6
        for i in 0..4 {
            assert!(r.types[0].rolls[i] >= 1);
            assert!(r.types[0].rolls[i] <= 6);
        }
        // assert the total is between 4 and 24
        assert!(r.types[0].total >= 4);
        assert!(r.types[0].total <= 24);
    }

    #[test]
    fn test_roll_6d6_4d8() {
        let d6 = DiceToRoll::new(6, Die::D6);
        let d8 = DiceToRoll::new(4, Die::D8);

        let mut v: Vec<DiceToRoll> = Vec::new();
        v.push(d6);
        v.push(d8);

        let r = roll_dice(v);

        assert!(r.types.len() == 2); // two types of dice
        assert!(r.types[0].die == Die::D6); // first die is d6
        assert!(r.types[0].rolls.len() == 6); // six d6 rolls

        // assert each d6 roll is between 1 and 6
        for i in 0..6 {
            assert!(r.types[0].rolls[i] >= 1);
            assert!(r.types[0].rolls[i] <= 6);
        }
        assert!(r.types[1].die == Die::D8); // second die is d8
        assert!(r.types[1].rolls.len() == 4); // four d8 rolls

        // assert each d8 roll is between 1 and 8
        for i in 0..4 {
            assert!(r.types[1].rolls[i] >= 1);
            assert!(r.types[1].rolls[i] <= 8);
        }

        // assert the total equals the subtotals
        assert!(r.total == r.types[0].total + r.types[1].total);
        // assert the total is between 10 and 68
        assert!(r.total >= 10);
        assert!(r.total <= 68);
    }
}
