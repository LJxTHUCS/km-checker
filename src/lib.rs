mod event;
mod kernel;
mod state;

pub use event::*;
pub use kernel::*;
pub use state::*;

#[cfg(test)]
mod test {
    use crate::*;

    #[derive(Debug)]
    struct EasyControlInfo {
        next_task: usize,
    }

    #[derive(Debug)]
    struct EasyState {
        tasks: IdentList<usize>,
        um: Unmatched<EasyControlInfo>,
    }

    // TODO: derive macro
    impl AbstractState for EasyState {
        fn matches(&self, other: &Self) -> bool {
            self.tasks.matches(&other.tasks) && self.um.matches(&other.um)
        }
    }

    struct Spawn;

    impl Event<EasyState> for Spawn {
        fn apply(&self, state: &mut EasyState) {
            state
                .tasks
                .0
                .push(Ident(state.um.0.next_task));
            state.um.0.next_task += 1;
        }
    }

    struct Sched;

    impl Event<EasyState> for Sched {
        fn apply(&self, state: &mut EasyState) {
            let head = state.tasks.0[0].clone();
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
            um: Unmatched(EasyControlInfo { next_task: 1 }),
        };
        let state1 = EasyState {
            tasks: IdentList(vec![Ident(100)]),
            um: Unmatched(EasyControlInfo { next_task: 101 }),
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
            kernel0.run(op);
            kernel1.run(op);
            assert!(kernel0.state.matches(&kernel1.state));
            println!("[0]{}: {:?}", op, kernel0.state);
            println!("[1]{}: {:?}", op, kernel1.state);
        }
    }
}
