use rand::{TryRngCore, rngs::OsRng};

fn main() {
    let mut tmp = [0u8; 20];
    OsRng.try_fill_bytes(&mut tmp).unwrap();
    println!(
        "{}",
        (0..5)
            .map(|i| (0..4)
                .map(|j| "0123456789abcdef".chars().nth((tmp[(i << 2) | j] & 0x0f).into()).unwrap())
                .collect())
            .collect::<Vec<String>>()
            .join("-")
    );
}
