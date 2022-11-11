// run-pass
#![feature(postfix_addr_of, postfix_deref, raw_ref_op)]

macro_rules! stringify_opaque_expr {
    ($e:expr) => {
        stringify!($e)
    };
}

fn main() {
    // FIXME: remove the space at the end
    assert_eq!(stringify_opaque_expr!(e.&), "e.&");
    assert_eq!(stringify_opaque_expr!(e.&mut), "e.&mut ");
    assert_eq!(stringify_opaque_expr!(e.&raw const), "e.&raw const ");
    assert_eq!(stringify_opaque_expr!(e.&raw mut), "e.&raw mut ");
}
