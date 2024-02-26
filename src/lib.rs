//! Sakura OS

fn ticks() -> u64 {
    1_000_000_000u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ticks() {
        assert_eq!(1_000_000_000u64, ticks());
    }
}
