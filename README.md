# rust-faq
a book about rust

# concept
this book will serve as a quick guideline for the frequent issues that you encounter when writing using rust

every chapter follows the flow like below
- topic question
- various implementation details
- comparison matrix of each implementation
    - pros and cons of each methodology

 the book presentation is closely analogous to cookbook series

# target audience
intermediate to advance rust developer

# table of contents

- which IDE to use
    - terminal based
        - VIM
        - neovim + Rust Analyzer
    - UI based
        - Visual Studio Code + Rust Analyzer
        - RustRover
- error handling
    - how to check what error type is return
    - using if let condition
    - using .unwrap or .expect
    - using `Box<dyn std::err::Error>`
    - using custom enum
    - from trait to convert error
    - using crates
        - anyhow
        - thiserror
- when to use what
    - .iter vs .into_iter
    - Box vs rc vs arc
    - Cell vs RefCell vs Mutex vs RwLock
    - String vs str
    - option vs results
    - Fn vs FnMut vs FnOnce vs fn
    - explicit vs infer type
        - .collect
    - functional vs imperative
    - `{:?}` vs `{:#?}`
    - lazy vs eager evaluation
        - https://stackoverflow.com/questions/65459952/what-is-the-difference-between-context-and-with-context-in-anyhow
- coding tips
    - stable sort
    - handling of writing long number
- what is this?
    - abc.0 , abc.1
    - match Some(abc) @ 1..3
    - declarative macros
    - procedural macros
- async await
- parser
    - simple if condition
    - regex
    - grammar
- state machine
    - object oriented approach
    - rust type approach
    - behavior tree
- splitting of module
    - single file (monofile)
    - splitting into multiple modules
- splitting to subcrates
    - motivation
        - the codebase gets too big and compilation time becomes long, subcrates are separate compilation units that can enhance speed
        - using procedural macros, because procedural macros can only work from a separate crate
- FFI
    - nodejs
    - c++
    - Python integration (PyO3)
- sample application using rust
    - blockchain
    - webserver
    - embedded
- mocking and testing
    - code coverage
    - parameterized testing
    - continuous event testing method
        - marble diagram
    - mocking
        - external crate
        - system I/O (network, file, time, etc)
- CI/CD
    - linting and formatting
    - build flags and optimization
    - cloud runner
        - github actions
    - version management
        - build environment
        - toolchain versions
        - package version
    - documentation
- packaging crate
   - as bin executable
       - deb package
       - windows?
   - as crate and publish registry
        - crate.io
        - github packages


book concept reference
- https://moodle.ufsc.br/pluginfile.php/2377667/mod_resource/content/0/Effective_Modern_C__.pdf
- https://isocpp.org/faq
