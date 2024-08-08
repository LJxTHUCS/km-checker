use crate::{AbstractState, Command, Error, ReadTargetMem, WriteTargetMem};
use std::{cell::RefCell, rc::Rc};

/// Communicate with the target kernel: send command and receive result.
pub trait TestPort<S>
where
    S: AbstractState,
{
    /// Send a command to the test target.
    fn send_command(&mut self, command: &dyn Command<S>) -> Result<(), Error>;
    /// Receive result from the test target.
    fn get_result(&mut self) -> isize;
}

/// Simple test port that writes command to and reads result from
/// target's virtual memory.
pub struct MemTestPort<R, W> {
    reader: R,
    writer: W,
    read_addr: usize,
    write_addr: usize,
}

impl<R, W> MemTestPort<R, W>
where
    R: ReadTargetMem,
    W: WriteTargetMem,
{
    pub fn new(reader: R, writer: W, read_addr: usize, write_addr: usize) -> Self {
        Self {
            reader,
            writer,
            read_addr,
            write_addr,
        }
    }
}

impl<S, R, W> TestPort<S> for MemTestPort<R, W>
where
    S: AbstractState,
    R: ReadTargetMem,
    W: WriteTargetMem,
{
    fn send_command(&mut self, command: &dyn Command<S>) -> Result<(), Error> {
        let buf = command.to_bytes();
        self.writer.write_virt(self.write_addr, &buf);
        Ok(())
    }
    fn get_result(&mut self) -> isize {
        let mut buf = [0u8; 8];
        self.reader.read_virt(self.read_addr, &mut buf);
        let retv = u64::from_le_bytes(buf) as isize;
        retv
    }
}

/// Fake test port that uses another state to emulate the test target.
pub struct FakeTestPort<S> {
    state: Rc<RefCell<S>>,
    result: isize,
}

impl<S> FakeTestPort<S> {
    /// Create a new fake test port.
    pub fn new(state: Rc<RefCell<S>>) -> Self {
        Self { state, result: 0 }
    }
}

impl<S> TestPort<S> for FakeTestPort<S>
where
    S: AbstractState,
{
    fn send_command(&mut self, command: &dyn Command<S>) -> Result<(), Error> {
        self.result = command.execute(&mut self.state.borrow_mut());
        Ok(())
    }
    fn get_result(&mut self) -> isize {
        self.result
    }
}
