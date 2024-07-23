# Kernel Model Check

A generic framework for **Model-Checking** of any customized kernel. 

## Abstract Model

### Build Model

A kernel model is a combination of abstract state and methods.

$$
M = (State, Methods)
$$

The only way to update a kernel model is executing a command

$$
\Pi(M,command) = M.exec(command)
$$

The resulting $\Pi(M, command)$ is a set that contains all possible model states after executing $command$.

For an abstract function $f$, a real kernel $K$ with state $S_k$, and a kernel model $M$ with state $S_M$, we say $S_M$ matches $S_K$ if

$$
S_M = f(S_K)
$$

For an execution process of a user app, the part that a kernel mostly focuses on can be abstracted as a command sequence.

$$
A = [command_1, command_2, \cdots]
$$

OS correctness can be marked as

$$
\begin{aligned}
K \sim M \iff & \forall A = [command_1,command_2,\cdots],\ S_{M,0} = f(S_{K,0}) \rightarrow  \\
&f(S_{K.exec(command_1)}) \in S_{\Pi(M,command_1)} \\
&\wedge f(S_{K.exec(command_1).exec(command_2)}) \in S_{\Pi(\Pi(K,command_1),command_2))} \\
&\wedge \cdots \\
\end{aligned}
$$

For each command sequence, after each execution step, the abstract state of the real kernel must be an element of all possible states of the kernel model.

### Test Routine

A normal test routine can be designed as

1. Match initial states, config kernel model such that

$$
S_M = f(S_K)
$$

2. Execute an command on both kernel and model, check if satisfies

$$
f(S_{K.exec(command)}) \in S_{\Pi(M,command)}
$$

3. If yes, update model state as

$$
S_M' \leftarrow f(S_K')
$$

4. Loop until a violation occurs.

   

## Implementation

1. Define an `AbstractState`.

   ```rust
   /// Generic Kernel State Type.
   pub trait AbstractState {
       fn matches(&self, other: &Self) -> bool;
   }
   ```

2. Implement `Command`s.

   ```rust
   /// A command that can be executed on a state.
   pub trait Command<T>
   where
       T: AbstractState,
   {
       /// Execute the command on the given state.
       fn execute(&self, state: &mut T) -> ExecutionResult;
       /// Serialize the command to a string.
       fn stringify(&self) -> String;
   }
   
   ```

3. Implement `Commander`, which sends commands to both kernel and model.

   ```rust
   /// Generate commands for both the abstract model and the target kernel.
   pub trait Commander<S>
   where
       S: AbstractState,
   {
       /// Get the next command to execute.
       fn command(&mut self) -> Result<Box<dyn Command<S>>, Error>;
   }
   ```

4. Implement `TestPort`, which communicates with the under-test kernel.

   ```rust
   /// Communicate with the target kernel.
   pub trait TestPort<S>
   where
       S: AbstractState,
   {
       /// Send a command to the test target.
       fn send(&mut self, command: &dyn Command<S>) -> Result<(), Error>;
       /// Receive the return value from the test target.
       fn receive_retv(&mut self) -> ExecutionResult;
       /// Receive current state from the test target.
       fn receive_state(&mut self) -> Result<&S, Error>;
   }
   ```

5. Combine all the modules in a `Runner`, call `step()` to run the test.

   ```rust
   /// Model Checking Runner.
   pub struct Runner<C, P, T, S>
   where
       C: Commander<S>,
       P: Printer<S>,
       T: TestPort<S>,
       S: AbstractState,
   {
       commander: C,
       printer: P,
       test_port: T,
       state: S,
   }
   
   impl<C, P, T, S> Runner<C, P, T, S>
   where
       C: Commander<S>,
       P: Printer<S>,
       T: TestPort<S>,
       S: AbstractState,
   {
       /// Construct a test runner.
       pub fn new(commander: C, printer: P, test_port: T, state: S) -> Self {
   		...
       }
   
       /// Run a single step of model checking.
       ///
       /// 1. Get command from commander
       /// 2. Send command to test port
       /// 3. Execute command on abstract state
       /// 4. Check return value (if enabled)
       /// 5. Check state (if enabled)
       ///
       /// `ReturnValueMismatch` if return value discrepancy is found.
       /// `StateMismatch` if state discrepancy is found.
       pub fn step(&mut self, check_retv: bool, check_state: bool) -> Result<(), Error> {
           ...
       }
   }
   ```



## Reference

* [A Practical Verification Framework for Preemptive OS Kernels](https://brightfu.github.io/research/certiucos/paper.pdf)
