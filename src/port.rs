use crate::{AbstractState, Command, Error, ReadTargetMem, WriteTargetMem};

/// Trait for sending commands and receiving results from a test target.
pub trait CommandChannel<S>
where
    S: AbstractState,
{
    /// Send a command to the test target.
    fn send_command(&mut self, command: &dyn Command<S>) -> Result<(), Error>;

    /// Receive return value from the test target.
    fn receive_retv(&mut self) -> isize;

    /// Receive other additional data from the test target.
    /// 
    /// Some commands may return additional data, such as some specific structure. Like
    /// `fstat` returns a `stat` structure, or `getdents` returns a list of directory entries.
    fn receive_data(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        Ok(vec![0u8; len])
    }
}

/// Trait for retrieving and managing the state of a test target.
pub trait StateChannel<S>
where
    S: AbstractState,
{
    /// Start the process of state retrieval.
    fn start_state_retrieval(&mut self) -> Result<(), Error>;

    /// Fetch state data from the test target.
    ///
    /// Return `Ok(false)` if there is more data to fetch.
    /// Return `Ok(true)` if all data has been fetched.
    fn retrieve_state_data(&mut self) -> Result<bool, Error>;

    /// Complete the state retrieval process and return the entire state.
    fn finish_state_retrieval(&mut self) -> Result<S, Error>;
}

/// A unified interface for interacting with a test target's state and commands.
///
/// 1. `CommandChannel`: Send commands and receive results.
/// 2. `StateChannel`: Manage and retrieve state from the test target.
pub trait TestPort<S>: CommandChannel<S> + StateChannel<S>
where
    S: AbstractState,
{
}

/// A mock implementation of `TestPort` that emulates a test target using an internal state.
pub struct MockTestPort<S> {
    state: S,
    result: isize,
}

impl<S> MockTestPort<S> {
    /// Create a new mock test port with the given initial state.
    pub fn new(state: S) -> Self {
        Self { state, result: 0 }
    }
}

impl<S> CommandChannel<S> for MockTestPort<S>
where
    S: AbstractState,
{
    fn send_command(&mut self, command: &dyn Command<S>) -> Result<(), Error> {
        self.result = command.execute(&mut self.state);
        Ok(())
    }
    fn receive_retv(&mut self) -> isize {
        self.result
    }
}

impl<S> StateChannel<S> for MockTestPort<S>
where
    S: AbstractState + Clone,
{
    fn start_state_retrieval(&mut self) -> Result<(), Error> {
        Ok(())
    }
    fn retrieve_state_data(&mut self) -> Result<bool, Error> {
        Ok(true)
    }
    fn finish_state_retrieval(&mut self) -> Result<S, Error> {
        Ok(self.state.clone())
    }
}

impl<S> TestPort<S> for MockTestPort<S> where S: AbstractState + Clone {}

/// Facilitates sending commands and receiving results via the target's virtual memory.
pub struct MemCommandChannel<R, W> {
    reader: R,
    writer: W,
    read_addr: usize,
    write_addr: usize,
    data_addr: Option<usize>
}

impl<R, W> MemCommandChannel<R, W>
where
    R: ReadTargetMem,
    W: WriteTargetMem,
{
    pub fn new(reader: R, writer: W, read_addr: usize, write_addr: usize, data_addr: Option<usize>) -> Self {
        Self {
            reader,
            writer,
            read_addr,
            write_addr,
            data_addr,
        }
    }
}

impl<S, R, W> CommandChannel<S> for MemCommandChannel<R, W>
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
    fn receive_retv(&mut self) -> isize {
        let mut buf = [0u8; 8];
        self.reader.read_virt(self.read_addr, &mut buf);
        let retv = u64::from_le_bytes(buf) as isize;
        retv
    }
    fn receive_data(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0u8; len];
        if let Some(data_addr) = self.data_addr {
            self.reader.read_virt(data_addr, &mut buf);
        }
        Ok(buf)
    }
}
