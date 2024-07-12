mod error;
mod event;
mod kernel;
mod runner;
mod state;

pub use event::*;
pub use kernel::*;
pub use runner::*;
pub use state::*;

#[cfg(test)]
mod test {
    use crate::*;
    use runner::{Runner, RunnerInput, RunnerOutput};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Default)]
    struct EasyControlInfo {
        next_task: usize,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct EasyState {
        tasks: IdentList<usize>,
        control: Unmatched<EasyControlInfo>,
    }

    // TODO: derive macro
    impl AbstractState for EasyState {
        fn matches(&self, other: &Self) -> bool {
            self.tasks.matches(&other.tasks) && self.control.matches(&other.control)
        }
    }

    struct Spawn;

    impl Event<EasyState> for Spawn {
        fn apply(&self, state: &mut EasyState) {
            state.tasks.0.push(Ident(state.control.0.next_task));
            state.control.0.next_task += 1;
        }
    }

    struct Sched;

    impl Event<EasyState> for Sched {
        fn apply(&self, state: &mut EasyState) {
            let head = state.tasks.0[0];
            state.tasks.0.remove(0);
            state.tasks.0.push(head);
        }
    }

    struct Exit;

    impl Event<EasyState> for Exit {
        fn apply(&self, state: &mut EasyState) {
            state.tasks.0.pop();
        }
    }

    #[test]
    fn test() {
        let state0 = EasyState {
            tasks: IdentList(vec![Ident(0)]),
            control: Unmatched(EasyControlInfo { next_task: 1 }),
        };
        let state1 = EasyState {
            tasks: IdentList(vec![Ident(100)]),
            control: Unmatched(EasyControlInfo { next_task: 101 }),
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
            kernel0.step(op);
            kernel1.step(op);
            assert!(kernel0.state.matches(&kernel1.state));
            println!("[0]{}: {:?}", op, kernel0.state);
            println!("[1]{}: {:?}", op, kernel1.state);
        }
    }

    struct RoundIn(usize);

    impl RunnerInput for RoundIn {
        fn read(&mut self) -> String {
            let ops = vec![
                "spawn", "sched", "sched", "spawn", "sched", "exit", "sched", "spawn", "exit",
                "exit",
            ];
            let res = ops[(self.0) % ops.len()].to_string();
            self.0 += 1;
            res
        }
    }

    struct Stdout;

    impl RunnerOutput for Stdout {
        fn write(&mut self, s: &str) {
            println!("{}", s);
        }
    }

    #[test]
    fn test_runner() {
        let state0 = EasyState {
            tasks: IdentList(vec![Ident(0)]),
            control: Unmatched(EasyControlInfo { next_task: 1 }),
        };
        let mut kernel0 = Kernel::new(state0);
        kernel0.register("spawn", Box::new(Spawn));
        kernel0.register("sched", Box::new(Sched));
        kernel0.register("exit", Box::new(Exit));
        let mut runner = Runner::new(RoundIn(0), Stdout, kernel0);
        for _ in 0..1000 {
            println!("=====================================");
            runner.step();
        }
    }
}
