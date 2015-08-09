General code layout:

```rust
main.rs:
 ├─ mod display // display.rs
 │   └─ struct Display // Used for turning images into strings of braille
 │       ├─ fn new(image, width, height) -> Self
 │       └─ fn render(&self) -> String
 │
 ├─ mod braille // braille.rs
 │   └─ make_braille(dotmap) -> char // Used exclusively by display.rs for turning pixels
 │                                   // into unicode braille characters
 │
 ├─ mod parse_args // parse_args.rs
 │   └─ struct ArgParser // Used by main() for parsing arguments
 │        ├─ fn new(args) -> Self
 │        ├─ fn path(&self) -> &String
 │        └─ fn size(&self) -> Option<(u32, u32)>
 │
 ├─ fn main() // Entry point. Deals with making a Display from the parsed arguments, and
 │            // sending the Display's render to stdout
 └─ fn complain(error) // Used by main() for erroring in a user-friendly way
```
