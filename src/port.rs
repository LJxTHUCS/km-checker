use crate::{AbstractState, Command, Error, ReadTargetMem, WriteTargetMem};

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
pub struct MemTestPort<S, R, W> {
    reader: R,
    writer: W,
    read_addr: usize,
    write_addr: usize,
    _state: std::marker::PhantomData<S>,
}

impl<S, R, W> MemTestPort<S, R, W>
where
    S: AbstractState,
    R: ReadTargetMem,
    W: WriteTargetMem,
{
    pub fn new(reader: R, writer: W, read_addr: usize, write_addr: usize) -> Self {
        Self {
            reader,
            writer,
            read_addr,
            write_addr,
            _state: std::marker::PhantomData,
        }
    }
}

impl<S, R, W> TestPort<S> for MemTestPort<S, R, W>
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
