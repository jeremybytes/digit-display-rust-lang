# digit-display-rust-lang
Naive hand-written digit recognition with display applications to show image, prediction, and errors.  

This is a Rust port of a project in .NET (F# & C#). Details of that project are available here: [https://github.com/jeremybytes/digit-display](https://github.com/jeremybytes/digit-display).  

This is primarily practice for Rust.

**Functions**  
* Reading files from the file system
* Training simple nearest-neighbor digit recognizers
    * Manhattan distance
    * Euclidean distance
* Output (pretty bad) ASCII art
* Multi-threading
* Channels
* Chunking / threading
* Parsing command-line parameters

**Usage**
```
PS C:\...> .\digits.exe --help
digits 1.0
Jeremy Clark
parses hand-written digits

USAGE:
    digits.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --classifier <classifier>    Classifier to use (default: 'euclidean')
    -c, --count <count>              Number of records to process (default: 100)
    -o, --offset <offset>            Offset in the data set (default: 1000)
    -t, --threads <threads>          Number of threads to use (default: 6)
```

*Update: Made this a lot faster by adding chunking, which reduces the number of threads to something more practical.*