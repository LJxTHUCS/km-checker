mod error;
mod event;
mod kernel;
mod runner;
mod state;

pub use error::*;
pub use event::*;
pub use kernel::*;
pub use runner::*;
pub use state::*;

#[cfg(feature = "derive")]
pub use derive::*;

#[cfg(test)]
mod test {
    use crate::*;
    use runner::{Commander, Printer, Runner};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Default)]
    struct EasyControlInfo {
        next_task: usize,
    }

    #[derive(Debug, Deserialize, Serialize, AbstractState)]
    struct EasyState {
        tasks: IdentList<usize>,
        #[serde(skip_serializing)]
        control: Ignored<EasyControlInfo>,
    }

    struct Spawn;

    impl Event<EasyState> for Spawn {
        fn apply(&self, state: &mut EasyState) -> Result<()> {
            state.tasks.0.push(state.control.0.next_task);
            state.control.0.next_task += 1;
            Ok(())
        }
    }

    struct Sched;

    impl Event<EasyState> for Sched {
        fn apply(&self, state: &mut EasyState) -> Result<()> {
            let head = state.tasks.0[0];
            state.tasks.0.remove(0);
            state.tasks.0.push(head);
            Ok(())
        }
    }

    struct Exit;

    impl Event<EasyState> for Exit {
        fn apply(&self, state: &mut EasyState) -> Result<()> {
            state.tasks.0.pop();
            Ok(())
        }
    }

    #[test]
    fn test() {
        let state0 = EasyState {
            tasks: IdentList(vec![0]),
            control: Ignored(EasyControlInfo { next_task: 1 }),
        };
        let state1 = EasyState {
            tasks: IdentList(vec![100]),
            control: Ignored(EasyControlInfo { next_task: 101 }),
        };
        let mut kernel0 = Kernel::new(state0);
        kernel0.register("spawn", Box::new(Spawn));
        kernel0.register("sched", Box::new(Sched));
        kernel0.register("exit", Box::new(Exit));
        let mut kernel1 = Kernel::new(state1);
        kernel1.register("spawn", Box::new(Spawn));
        kernel1.register("sched", Box::new(Sched));
        kernel1.register("exit", Box::new(Exit));

        let ops = vec![
            "spawn", "sched", "sched", "spawn", "sched", "exit", "sched", "spawn", "sched", "exit",
        ];
        for op in ops {
            kernel0.step(op).expect("kernel0.step failed");
            kernel1.step(op).expect("kernel1.step failed");
            assert!(kernel0.state.matches(&kernel1.state));
            println!("[0]{}: {:?}", op, kernel0.state);
            println!("[1]{}: {:?}", op, kernel1.state);
        }
    }

    struct RoundIn(usize);

    impl Commander for RoundIn {
        fn command(&mut self) -> Result<String> {
            let ops = vec![
                "spawn", "sched", "sched", "spawn", "sched", "exit", "sched", "spawn", "exit",
                "exit",
            ];
            let res = ops[(self.0) % ops.len()].to_string();
            self.0 += 1;
            Ok(res)
        }
    }

    struct Stdout;

    impl Printer<EasyState> for Stdout {
        fn print_str(&mut self, s: &str) -> Result<()> {
            println!("{}", s);
            Ok(())
        }
        fn print_state(&mut self, s: &EasyState) -> Result<()> {
            let sta_str =
                serde_json::to_string(&s).map_err(|_| Error::new(ErrorKind::StateParseError))?;
            println!("{}", sta_str);
            Ok(())
        }
    }

    struct FakeTestPort(Kernel<EasyState>);

    impl TestPort<EasyState> for FakeTestPort {
        fn send(&mut self, event: &str) -> Result<()> {
            // Random error
            // if rand::random::<u64>() % 100 == 0 {
            //     return;
            // }
            self.0.step(event)
        }
        fn receive(&mut self) -> Result<&EasyState> {
            let sta_str = serde_json::to_string(&self.0.state)
                .map_err(|_| Error::new(ErrorKind::StateParseError))?;
            let _sta = serde_json::from_str::<EasyState>(&sta_str)
                .map_err(|_| Error::new(ErrorKind::StateParseError))?;
            Ok(&self.0.state)
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
        let mut kernel0 = Kernel::new(state0);
        kernel0.register("spawn", Box::new(Spawn));
        kernel0.register("sched", Box::new(Sched));
        kernel0.register("exit", Box::new(Exit));
        let mut kernel1 = Kernel::new(state1);
        kernel1.register("spawn", Box::new(Spawn));
        kernel1.register("sched", Box::new(Sched));
        kernel1.register("exit", Box::new(Exit));

        let mut runner = Runner::new(RoundIn(0), Stdout, FakeTestPort(kernel1), kernel0);
        for _ in 0..1000 {
            println!("=====================================");
            runner.step().expect("Runner Exited");
        }
    }
}
