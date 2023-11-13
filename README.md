<div align="center">
  <img src="https://github.com/Nukleus-Language/nukleus/blob/main/images/logo.png" alt="Nukleus Logo" width="140" height="140"></img>
</div>

# Welcome to Nukleus: Redefining Simplicity and Speed in Programming
[![Rust](https://github.com/Nukleus-Language/nukleus/actions/workflows/rust.yml/badge.svg)](https://github.com/Nukleus-Language/nukleus/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/Nukleus-Language/nukleus/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Nukleus-Language/nukleus/actions/workflows/rust-clippy.yml)
[![Build Status](https://drone.nornity.com/api/badges/Nukleus-Language/nukleus/status.svg)](https://drone.nornity.com/Nukleus-Language/nukleus)

Introducing Nukleus, a revolutionary programming language designed with a focus on AI, GUI, and cross-platform development. Underpinned by the robustness of Rust, Nukleus promises high performance, reliability, and ease of maintenance.

## Project Status
While Nukleus is still in its early stages, our team is devoted to bringing an array of exciting features to life. Stay tuned for the journey ahead!
<!-- We're just getting warmed up! -->

- [ ] Under Development
- [x] Live Now


## Current Compiler Capabilities

Nukleus' current compiler is a basic yet robust tool designed for early-stage development. Here’s what it supports as of now:

- **Recursive Function Calls**: Enables functions to call themselves, facilitating complex computations like the Fibonacci sequence.
- **Operator Precedence**: Ensures mathematical expressions are evaluated in the correct order, respecting standard mathematical rules.
- **Control Structures**: Includes `if` and `else` for conditional execution, and `for` loops for iterative operations.

### Limitations

- **No 'Print' Support**: Currently, the compiler lacks the ability to output results directly to the console or any other output stream.
- **Sample Code Execution**: The compiler is optimized to run specific sample codes like `Fibonacci.nk`.

```rust
fn fibonacci(i64:n) -> i64 {
    let:i64 return_val = 0;
    if(n < 2) {
        return_val = n;
    } else {
        return_val = fibonacci( n-1 ) + fibonacci( n-2 );
    }
    return return_val;
}

fn main() -> i64 {
    let:i64 x = 47;
    return fibonacci(x);
}
```

This limitation is a temporary measure as we continue to refine and expand the compiler’s capabilities.

### Interpreter Versatility

In contrast, the experimental interpreter can run a wider range of Nukleus code, showcasing the language's potential and flexibility.

## Features on the Horizon

- [x] Intuitive syntax influenced by the elegance of modern programming languages
- [ ] Top-notch efficiency powered by the might of Rust
- [ ] Versatility that stands out
	+ [x] Cross-platform capabilities
	+ [ ] Integrated GUI toolkit
	+ [ ] Comprehensive AI toolkit
- [x] Strongly typed language ensuring enhanced reliability and maintainability
- [ ] Rich and extensive standard library
- [ ] Expandability with Rust libraries
- [ ] User-centric documentation filled with practical examples

## Your First Steps with Nukleus

Welcome to Nukleus! Here's how to get started:

### 1. Set Up Your Workspace
- **Download the Nukleus Compiler/Interpreter**: You need to build the project locally for the moment.

### 2. Try Out Sample Programs
- **Access the Example Repository**: Our [GitHub repository](#) has a variety of sample programs for you to try.
- **Run and Modify**: Start by running these examples and then experiment by making changes.

### 3. Write Your First Program
- **Create Your Code**: Use your new knowledge to write your own Nukleus program.namic community!
