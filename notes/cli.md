## Cargo
Cargo is the package & build manager.

Commands
***
`cargo new <<project>>`
- creates base project with src/ directory and  Cargo.toml file
- src will have main.rs
- Cargo.toml is like other package management files such as Pipefile, Gemfile, etc...

`cargo check`
- checks if can be compiled but doesnt build the executable
- takes less timw than cargo build

`cargo build`
- compiles and builds executable (./target)
- this does more than just rustc which is why it takes longer (typically)
- it actually checks whether it should be compiled at all so if its run multiple times the time to run is shorter

`cargo run`
- builds then runs the project

`rustc`
- compiler, will compile irregardless of whether it was already compiled
