trait Memory {
    fn get(address: &u16) -> u8;

    /// Change the value of the address
    fn set(address: &u16, value: u8);
}