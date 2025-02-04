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

# book writing stages
- write all sample code
- write guide based on sample code
- refine

# target audience
intermediate to advance rust developer

# table of contents

- [ ] which IDE to use
    - [ ] terminal based
        - [ ] VIM
        - [ ] neovim + Rust Analyzer
    - [ ] UI based
        - [ ] Visual Studio Code + Rust Analyzer
        - [ ] RustRover
- [X] error handling
    - [X] how to check what error type is return
    - [X] using if let condition
    - [X] using .unwrap or .expect
    - [X] using `Box<dyn std::err::Error>`
    - [X] using custom enum
    - [X] from trait to convert error
    - [X] using crates
        - [X] anyhow
        - [X] thiserror
- [ ] when to use what
    - [ ] .iter vs .into_iter
    - [ ] Box vs rc vs arc
    - [ ] Cell vs RefCell vs Mutex vs RwLock
    - [ ] String vs str
    - [ ] iter vs element index access
    - [ ] option vs results
    - [ ] Fn vs FnMut vs FnOnce vs fn
    - [ ] explicit vs infer type
        - [ ] .collect
    - [ ] functional vs imperative
    - [ ] `{:?}` vs `{:#?}`
    - [ ] lazy vs eager evaluation
        - [ ] how to know a function is lazy evaluated?
        - [ ] how to implement lazy evaluated fn?
        - [ ] example of lazy evaluation fn
            - [ ] ok_or vs ok_or_else
        - [ ] reference
            - [ ] https://deepu.tech/functional-programming-in-rust/
            - [ ] https://stackoverflow.com/questions/65459952/what-is-the-difference-between-context-and-with-context-in-anyhow
- [X] coding tips
    - [X] how to sort
        - [X] 1D
        - [X] 2D
            - [X] stable
            - [X] unstable
    - [X] how to input multiple test sample
    - [X] how to write long number
    - [X] how to create chaining function
    - [X] how to ternary operator
    - [X] how to handle safe arithmetic operation
    - [X] how to handle large test sample
    - [X] how to handle unit in variables
    - [X] how to write string variables
- [ ] what is this?
    - [ ] abc.0 , abc.1
    - [ ] match Some(abc) @ 1..3
    - [ ] declarative macros
    - [ ] procedural macros
- [ ] async await
- [X] parser
    - [X] simple if condition
    - [X] regex
    - [X] grammar (nom)
- [ ] state machine
    - [ ] object oriented approach
    - [ ] rust type approach
    - [ ] behavior tree
- [ ] splitting of module
    - [ ] single file (monofile)
    - [ ] splitting into multiple modules
- [ ] splitting to subcrates
    - [ ] motivation
        - [ ] the codebase gets too big and compilation time becomes long, subcrates are separate compilation units that can enhance speed
        - [ ] using procedural macros, because procedural macros can only work from a separate crate
- [ ] FFI
    - [ ] nodejs
    - [ ] c++
    - [ ] Python integration (PyO3)
- [ ] sample application using rust
    - [ ] blockchain
    - [ ] webserver
    - [ ] embedded
- [ ] mocking and testing
    - [ ] code coverage
    - [ ] parameterized testing
    - [ ] continuous event testing method
        - [ ] marble diagram
    - [ ] mocking
        - [ ] external crate
        - [ ] system I/O (network, file, time, etc)
- [ ] CI/CD
    - [ ] linting and formatting
    - [ ] build flags and optimization
    - [ ] cloud runner
        - [ ] github actions
    - [ ] version management
        - [ ] build environment
        - [ ] toolchain versions
        - [ ] package version
    - [ ] documentation
- [ ] packaging crate
   - [ ] as bin executable
       - [ ] deb package
       - [ ] windows?
   - [ ] as crate and publish registry
        - [ ] crate.io
        - [ ] github packages


book concept reference
- https://moodle.ufsc.br/pluginfile.php/2377667/mod_resource/content/0/Effective_Modern_C__.pdf
- https://isocpp.org/faq
