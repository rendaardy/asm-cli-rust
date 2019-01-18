pub mod machine;

use unicorn::Cpu;

use crate::machine::interface::Interface;
fn get_trait() -> impl Interface {
    let m = machine::x64::X64Machine::new();
    return m;
}

fn main() {
    let m = get_trait();
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                let result = m.asm(input.to_string(),0);
                match result {
                    Ok(r) => {
                        m.write_instruction(r.bytes);
                        m.print_register()
                    }
                    Err(e) => println!("failed to assemble, err: {:?}", e),
                }
            }
            Err(error) => println!("error when read your input: {}", error),
        }
        println!();
    }
}
