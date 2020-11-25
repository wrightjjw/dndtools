use rand::Rng;

/// Generate a block of PC stats.
///
/// This is done by calculating each stat by rolling four d6 and dropping the lowest.
/// Returns an array of `u8`.
pub fn gen_stats() -> [u8; 6] {
    let mut stats: [u8; 6] = [0; 6];
    let mut rng = rand::thread_rng();

    let mut i = 0;
    while i < 6 {
        let mut rolls: [u8; 4] = [0; 4];
        let mut j = 0;
        while j < 4 {
            rolls[j] = rng.gen_range(1, 7);
            j += 1;
        }
        rolls.sort();
        j = 0;
        while j < 3 {
            stats[i] += rolls[j];
            j += 1;
        }
        i += 1;
    }
    stats.sort();
    return stats;
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
