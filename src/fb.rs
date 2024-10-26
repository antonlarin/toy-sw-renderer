pub trait Frambuffer {
    fn width(&self) -> i32;
    fn heigh(&self) -> i32;

    fn set_pixel(&mut self, x: i32, y: i32) -> Result<(), SomeError>;
}
