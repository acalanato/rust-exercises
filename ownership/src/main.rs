/*
Stack (pilha)
last in, first out
add data -> push
rem data -> pop
all data must have fixed, know size

Heap (malloc)
Slower than stack, but can be stored in the heap
Size can be unknow at compiler time
Use pointers to find data
Values are popped off when the function is over

Ownership Rules
· Each value has an owner
· There can be only one (at a time)
· When the owner goes out of scope, the value will be dropped
*/
fn main() {
    {                                   // nothing to see here
	let s = String::from("hello?"); // s has been born
	println!("{s}");                // s still available to work
    }                                   // too bad, s is gone
}
