// check-pass
#![feature(postfix_addr_of, postfix_deref, raw_ref_op)]

struct F;

impl std::ops::BitAnd<F> for f32 {
    type Output = ();

    fn bitand(self, F: F) {}
}

impl std::ops::Mul<F> for f32 {
    type Output = ();

    fn mul(self, F: F) {}
}

fn main() {
    // Check that `.&` and `.*` operators don't have priority over `N.` float literals
    // N.B.: there should be no space between . and &/* to actually test this
    0.&F;
    0.*F;
}
