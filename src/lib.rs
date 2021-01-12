pub trait Version: Sized + std::str::FromStr {
    fn minor(&self) -> u32;
    fn major(&self) -> u32;
    fn patch(&self) -> u32 {
        0
    }
}
