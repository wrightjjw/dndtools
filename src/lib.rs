use rand::Rng;

/// Enum for type of die.
#[derive(Clone, Copy)]
pub enum Die {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

/// Struct to represent multiple rolls of a single die type.
pub struct Rolls {
    die: Die,
    rolls: u32,
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
/// Each individual roll is calculated and returned.
/// Returns a tuple containing a Vec of inner tuples and an overall sum.
/// The inner tuples contain the die type and individual roll values of the die type.
pub fn roll_dice(rolls: Vec<Rolls>) -> (Vec<(Die, Vec<u32>)>, u32) {
    let mut total: u32 = 0;
    let mut ret: Vec<(Die, Vec<u32>)> = Vec::new();
    let mut rng = rand::thread_rng();

    // die_type represents each type of die given as a parameter
    for die_type in rolls.iter() {
        // die_rolls is a vector containing each individual roll of the die type (inner vecs of return)
        let mut die_rolls: Vec<u32> = Vec::new();
        for _ in 0..die_type.rolls {
            // this_roll is the value of an individual roll
            let this_roll: u32 = rng.gen_range(1, die_type.die as u32 + 1);
            die_rolls.push(this_roll);
            total = total + this_roll;
        }
        ret.push((die_type.die, die_rolls));
    }

    return (ret, total);
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
}
