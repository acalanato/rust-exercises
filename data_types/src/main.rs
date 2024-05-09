/*
<integers>
Lengh    Signed    Unsigned
8-bit    i8        u8
16-bit   i16       u16
32-bit   i32       u32
64-bit   i64       u34
128-bit  i128      u128
arch     isize     usize
signed inclui negativos
unsigned apenas positivos

Number literals	Example
Decimal	        98_222
Hex	        0xff
Octal	        0o77
Binary	        0b1111_0000
Byte (u8 only)	b'A'

In debug mode, overflow generate a panic!
In release mode, wraping ocurr.

Float points
f32 -> 32bits
f64 -> 64bits
*/

fn main() {
    let x = float_point().0;
    let y = float_point().1;
    println!("Float 64 is {x} and 32 is {y}");

    let add = math("div",5,10);
    println!("Add result is {add}");
}

fn float_point() -> (f64, f32) {
    let x = 2.1; // f64
    let y: f32 = 3.2;
    return (x,y)
}

fn math(operation: &str, a: i32, b: i32) -> i32 {
    match operation{
	add => a + b,
	div => a / b,
	sub => a - b,
	mul => a * b,
	_=> panic!{""}
    }
}
