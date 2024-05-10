// use std::fmt; part of impl

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

Boolean can be either true or false:
let t = true;
let f: bool = false;

char is a single character with single quotes ''
let z: char = 'Z'

 */

fn main() {
    let x = float_point().0;
    let y = float_point().1;
    println!("Float 64 is {x} and 32 is {y}");

    let add = math(1,5.0,10.0);
    let div = math(2,56.7,32.2);
    let sub = math(3,95.5,4.3);
    let pro = math(4,4.0,30.0);
    println!("Add result is {add}");
    println!("Division result is {div}");
    println!("Subtraction result is {sub}");
    println!("Product result is {pro}");
    println!("Addition from module is {}", op::add(1,2));
    let struc = Struc {x: 500, y: 6.4, z: 1};
    let tuple = (500, 6.4, 1);
    let (_a, _b, _c) = tuple;
    println!("x: {}, y:{} and z:{}",struc.x, struc.y, struc.z);
    println!("Indirect access of the tuple:{_b}");
    println!("Direct access of the tuple is dot + index:{}", tuple.1);
    let arr = Arr();
    println!("Direct access of the array element is [index]:{}", arr[3]);
}

fn float_point() -> (f64, f32) {
    let x = 2.1; // f64
    let y: f32 = 3.2;
    return (x,y)
}

mod op {
    pub fn add (a: i32, b: i32) -> i32 {
	a + b
    }
    pub fn sub (a:i32 , b: i32) -> i32 {
	a - b
    }
}

fn math(operation: u8, a: f32, b: f32) -> f32 {
    match operation{
	1 => a + b,
	2 => a / b,
	3 => a - b,
	4 => a * b,
	_=> panic!{"Learn to code properly"}
    }
}

struct Struc {
    x: i32,
    y: f64,
    z: u8
}

fn Arr() -> [i32;5] {
    let a: [i32; 5] = [1,2,3,4,5];
    return a;
}

/*
impl fmt::Display for Tup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "Tuple is {}", self.x)
    }
}
*/
