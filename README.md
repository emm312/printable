# printable
Printable is a QOL crate to add member functions to print objects that implement `Display`.

## Examples
```rs
use printable::DisplayPrintable;

fn main() {
    let a = 6;
    a.print();
    9.println();
}
```

```rs
use printable::*;

#[derive(Debug)]
struct Testy(i32, i64);

fn main() {
    let t = Testy(2, 3);
    t.debug().println();
}
```