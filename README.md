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
* Multi-threading
* Channels

*Note: The threading is pretty ugly, and things are pretty slow right now. But it works. It definitely needs optimization.*
