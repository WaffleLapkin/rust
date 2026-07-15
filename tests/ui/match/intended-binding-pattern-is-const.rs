fn main() {
    match 1 { //~ ERROR non-exhaustive patterns
        //~| NOTE the matched value is of type `i32`
        x => {} //~ NOTE this pattern doesn't introduce a new catch-all binding
        //~^ HELP ensure that all possible cases are being handled
        //~| HELP if you meant to introduce a binding, use a different name
    }
    const x: i32 = 4; //~ NOTE constant `x` defined here
}
