use jlrs::prelude::*;

fn main() {
    // Initializing Julia is unsafe because this can load arbitrary
    // Julia code, and because it can race with other crates unrelated
    // to jlrs. It returns an error if Julia has already been
    // initialized.
    let mut julia = unsafe { RuntimeBuilder::new().start().unwrap() };

    // A StackFrame must be provided to ensure Julia's GC can be made aware
    // of references to Julia data that exist in Rust code.
    let mut frame = StackFrame::new();
    let _instance = julia.instance(&mut frame);
}
