# Symbolically Executable Stack Machines in Rust
Symbolic Stack Machines is a library for implementing symbolically executable stack-based virtual machines.

This works by providing various traits for a `Stack`, `Memory` (including read-only and writeable memory), `VMInstruction`,
as well as generic `Val` structure to represent stack or memory values. Finally, it provides a set of abstract `Machines`
which can be instantiated with a specific instruction set and can execute a sequence of instructions concretely or symbolically.

Implementing a new VM involves, simply, implementing the `VMInstruction` trait for an enum that describes the instruction set of the
operations.

Each operation should output a state diff (`ExecRecord`), which describes the updates to the stack, memory, any new path constraints (in the case of symbolic execution), and machine's program counter. The machine can apply the update with `ExecRecord::apply`.

# Reachability

The Machine constructs the updates performed on its state by instructions. Operations can add path constraints via the ExecRecord structure. The Machine incrementally constructs a set of constraints for each possible path of execution. Once these paths are constructed, the machine checks whether such paths are reachable. See `src/machine/mod.rs`, specifically, the `BaseMachine::run_sym` method for the implementation.

Currently, there is no intermediate path pruning performed.

# Supported Memory & Stack Models
Currently, two forms of symbolic memory are built-in: Memory based on the theory of arrays, and finite concrete memory that can store possibly symbolic values.

The only built-in stack model right now is a finite stack that can store symbolic or concrete values.

The only symbolic values that have built-in support right now are integers.

# Usage
See `lib.rs` for a toy instruction set and its symbolic execution.

# Up Next
WIP features:
- Stack based on theory of arrays
- Support for memory model based on uninterpreted functions
- Support for bitvector values, stack and memory
- Try to stop calling `unwrap()` on every single result type kek 