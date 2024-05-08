const C: u32 = 5;

fn shadowing(s: &str) -> u32 {
    let mut spaces = String::from(s);
//    let mut spaces = s;
    let spaces = spaces.len();
    return spaces.try_into().unwrap()
}

fn main() {
    let mut x = C;
    println!("O valor de x é: {x}");
    x = 6;

    {
	let x = x * 2;
	println!("O valor de x no escopo interno é: {x}");
    }

        println!("O valor de x mudou para: {x}");
    println!("Shadowing: {}", shadowing("pra_transformar_em_u32"))
}

