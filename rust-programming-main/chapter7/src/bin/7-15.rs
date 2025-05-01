use std::fmt;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
