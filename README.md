# digit-display-rust-lang
Naive hand-written digit recognition with display applications to show image, prediction, and errors.  

This is a Rust port of a project in .NET (F# & C#). Details of that project are available here: [https://github.com/jeremybytes/digit-display](https://github.com/jeremybytes/digit-display).  

This is primarily practice for Rust.

Functions:  
* Reading files from the file system
* Training simple nearest-neighbor digit recognizers
    * Manhattan distance
    * Euclidean distance
* Output (pretty bad) ASCII art

*Note: I hit a breaking point: threading. The ownership model in Rust is giving me a bit of a problem when I try to spin up threads and use channels to communicate. Still working on a solution.*
