use crate::{AbstractState, Kernel};

pub trait RunnerInput {
    fn read(&mut self) -> String;
}

pub trait RunnerOutput {
    fn write(&mut self, s: &str);
}
pub struct Runner<I, O, S>
where
    I: RunnerInput,
    O: RunnerOutput,
    S: AbstractState,
{
    input: I,
    output: O,
    kernel: Kernel<S>,
}

impl<I, O, S> Runner<I, O, S>
where
    I: RunnerInput,
    O: RunnerOutput,
    S: AbstractState,
{
    pub fn new(input: I, output: O, kernel: Kernel<S>) -> Self {
        Self {
            input,
            output,
            kernel,
        }
    }
    pub fn step(&mut self) {
        let event = self.input.read();
        self.kernel.step(&event);
        let output = format!("{:?}", serde_json::to_string(&self.kernel.state).unwrap());
        self.output.write(&output)
    }
}
