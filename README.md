# Rust Competetive Programming Template

This is my personal template for when solving small problems in Rust.

The idea is that you copy the template when starting a new project with `copy-templ.sh /path/to/new/project` (can be called from outside this folder). Then you solve the problem in _main.rs_ using helper functions, such as parsing a String iterator to integers, from the included library. Finally, if you want to submit the code to a website, you can run _bundle.sh_ to bundle all of _src/_ into a single _bundled.rs_ source code file.
