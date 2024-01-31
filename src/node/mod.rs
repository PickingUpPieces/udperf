use crate::util::IOModel;

pub mod client;
pub mod server;

pub trait Node {
    fn run(&mut self, io_model: IOModel) -> Result<(), &'static str>;
    fn loop_busy_waiting(&mut self) -> Result<(), &'static str>;
    fn loop_select(&mut self) -> Result<(), &'static str>;
    fn loop_poll(&mut self) -> Result<(), &'static str>;
}
