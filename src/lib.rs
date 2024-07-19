mod command;
mod error;
mod runner;
mod state;

pub use command::*;
pub use error::*;
pub use runner::*;
pub use state::*;

#[cfg(feature = "derive")]
pub use derive::*;

#[cfg(test)]
mod test {
    use crate::*;
    use runner::{Commander, Printer, Runner};

    #[derive(Debug, Default)]
    struct EasyControlInfo {
        next_task: usize,
    }

    #[derive(Debug, AbstractState)]
    struct EasyState {
        tasks: IdentList<usize>,
        control: Ignored<EasyControlInfo>,
    }

    struct Spawn;

    impl Command<EasyState> for Spawn {
        fn execute(&self, state: &mut EasyState) -> Result<usize> {
            state.tasks.0.push(state.control.0.next_task);
            state.control.0.next_task += 1;
            Ok(0)
        }
        fn stringify(&self) -> String {
            "spawn".to_string()
        }
    }

    struct Sched;

    impl Command<EasyState> for Sched {
        fn execute(&self, state: &mut EasyState) -> Result<usize> {
            let head = state.tasks.0[0];
            state.tasks.0.remove(0);
            state.tasks.0.push(head);
            Ok(0)
        }
        fn stringify(&self) -> String {
            "sched".to_string()
        }
    }

    struct Exit;

    impl Command<EasyState> for Exit {
        fn execute(&self, state: &mut EasyState) -> Result<usize> {
            state.tasks.0.pop();
            Ok(0)
        }
        fn stringify(&self) -> String {
            "exit".to_string()
        }
    }

    struct RoundIn(usize);

    impl Commander<EasyState> for RoundIn {
        fn command(&mut self) -> Result<Box<dyn Command<EasyState>>> {
            let ops = vec![
                "spawn", "sched", "sched", "spawn", "sched", "exit", "sched", "spawn", "exit",
                "exit",
            ];
            let res = ops[(self.0) % ops.len()].to_string();
            self.0 += 1;
            match res.as_str() {
                "spawn" => Ok(Box::new(Spawn)),
                "sched" => Ok(Box::new(Sched)),
                "exit" => Ok(Box::new(Exit)),
                _ => Err(Error::new(ErrorKind::CommandNotFound)),
            }
        }
    }

    struct Stdout;

    impl Printer<EasyState> for Stdout {
        fn print_str(&mut self, s: &str) -> Result<()> {
            println!("{}", s);
            Ok(())
        }
        fn print_state(&mut self, s: &EasyState) -> Result<()> {
            println!("{:?}", s);
            Ok(())
        }
    }

    struct FakeTestPort(EasyState);

    impl TestPort<EasyState> for FakeTestPort {
        fn send(&mut self, command: &dyn Command<EasyState>) -> Result<()> {
            command.execute(&mut self.0).map(|_| ())
        }
        fn receive_state(&mut self) -> Result<&EasyState> {
            Ok(&self.0)
        }
        fn receive_retv(&mut self) -> Result<usize> {
            Ok(0)
        }
    }

    #[test]
    fn test_runner() {
        let state0 = EasyState {
            tasks: IdentList(vec![0]),
            control: Ignored(EasyControlInfo { next_task: 1 }),
        };
        let state1 = EasyState {
            tasks: IdentList(vec![100]),
            control: Ignored(EasyControlInfo { next_task: 101 }),
        };
        let mut runner = Runner::new(RoundIn(0), Stdout, FakeTestPort(state1), state0);
        for _ in 0..1000 {
            println!("=====================================");
            runner.step(true, true).expect("Runner Exited");
        }
    }
}
