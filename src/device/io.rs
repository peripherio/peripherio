pub trait Interface {
    fn name(&self) -> String;
    fn speed(&self) -> u32;
    fn set_speed(&self);
}
