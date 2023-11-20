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
 
# table of contents

- which IDE to use
- error handling
    - using if let condition
    - using .unwrap or .expect
    - using Box<dyn std::err:Error> 
    - using custom enum
    - using crates
        - anyhow
        - thiserror
- state machine
    - object oriented approach
    - rust type approach
    - behavior tree 
- handling of writing long number
- splitting of module 
    - single file (monofile)
    - splitting into multiple modules
- FFI
    - nodejs
    - c++
- application using rust
    - blockchain
    - webserver
    - embedded
- mocking and testing
    - code coverage
    - parameterized testing
    - mocking
        - external crate
        - system I/O (network, file, time, etc)
- build flags and optimization
- CI/CD
    - github actions
- packaging crate
   - as bin executable
       - deb package
       - windows?
   - as crate and publish to crate.io
