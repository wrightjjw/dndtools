use rand::Rng;

fn genStats() {
    let args: Vec<String> = env::args().collect();
    let mut stats: [u8; 6] = [0; 6];
    let mut rng = rand::thread_rng();

    let mut i = 0;
    while i < 6 {
        let mut rolls: [u8; 4] = [0; 4];
        let mut j = 0;
        while j < 4 {
            rolls[j] = rng.gen_range(1, 6);
            j += 1;
        }
        rolls.sort();
        j = 3;
        while j > 0 {
            stats[i] += rolls[j];
        }
        i += 1;
    }
}
