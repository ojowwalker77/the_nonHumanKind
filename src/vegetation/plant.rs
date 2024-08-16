pub trait Plant {
    fn update(&mut self);
    fn can_be_eaten(&self) -> bool;
    fn eat(&mut self) -> u32;
}
