use printable::{DisplayPrintable, MDebug};

#[derive(Debug)]
struct Testy(i32, i64);

fn main() {
    let t = Testy(2, 3);
    t.debug().println();
}
