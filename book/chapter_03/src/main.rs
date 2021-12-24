fn main() {
    let mut s = String::from("hello");

    // since the values is on the heap, it can be mutated
    s.push_str(", world");
    println!("{}", s);

    // this is borrow, not a move
    let k = &s;

    // this is a move, since String has no Copy trait
    // let k = s;

    // this does not work, since a mutable value can only be borrowed once
    // s.push_str(", world");

    println!("{}", k);
    println!("{}", s)

    // in rust, you can have infinitely many immutable borrows but just one singular mutable borrow
}
