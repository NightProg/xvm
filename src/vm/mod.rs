pub mod cpu;
mod mem;
mod segment;
mod register;
mod virtualdisk;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Real,
    Protected,
    Long,
}
