RISC-V Simulator Code Overview - Deep Dive

1. Memory Module (mem.rs)
Functionality:

Manages memory operations with a Vec<i32> for data storage.
Key methods:
load(address) fetches the value at a given address.
store(address, value) updates the memory at the specified address.
Improvements:

Add boundary checks for invalid addresses to prevent out-of-bounds errors.
2. Cache Module (cache.rs)
Functionality:

Implements a cache with data storage (Vec<Option<i32>>), validity flags (Vec<bool>), and an LRU mechanism (Vec<usize>).
Key methods:
load(address) retrieves a value if valid; otherwise, falls back to memory.
store(address, value) updates the cache and validity flags.
Improvements:

Enhance the LRU mechanism to avoid overflow.
Add support for advanced cache policies (e.g., associative mapping).
3. CPU Module (cpu.rs)
Functionality:

Represents the CPU with 32 general-purpose registers and a program counter (pc).
Improvements:

Add debugging utilities to print the current state of registers and the program counter.
4. Instruction Module (instr.rs)
Functionality:

Defines supported instructions (Add, Sub, Load, Store, Jump, Halt).
Encapsulates RISC-V operations.
Improvements:

Implement string representation for instructions to aid debugging.
Integration (main.rs)
Functionality:

Parses and executes instructions step-by-step.

Manages interactions between the CPU, Memory, and Cache modules.

Suggested Enhancements
Interactive Debugging Mode: Allow users to step through instructions, observing the CPU and memory state after each step.
Logging and Metrics: Track execution statistics such as instruction count and cache hit/miss rates.
Error Handling: Improve error handling for invalid instructions and memory addresses.
Unit Tests: Add test cases to validate functionality of all modules.

Example Enhancements
Debugging Utilities:
```rust
Copy code
impl Cpu {
    pub fn dump_registers(&self) {
        for (i, reg) in self.registers.iter().enumerate() {
            println!("x{}: {}", i, reg);
        }
    }
    pub fn dump_pc(&self) {
        println!("Program Counter: {}", self.pc);
    }
}
```
Interactive Execution:
```rust
for instr in instructions {
    println!("Executing: {}", instr.to_string());
    execute_instruction(&mut cpu, &mut memory, &mut cache, &instr);
    cpu.dump_registers();
    cpu.dump_pc();
    println!("Press Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
```
This modular and extensible design is ideal for simulating RISC-V assembly programs while offering plenty of room for enhancements.

### Author
Ben Santora
