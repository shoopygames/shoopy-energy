use std::io::{Write};

pub fn is_valid_address(addr: &str) -> Result<&str, &'static str> {
    if !addr.chars().all(|c| c.is_ascii_alphanumeric())
    {
        return Err("Invalid Cell Address!\r\nhttps://shoopy.ir/");
    }
    Ok(addr)
}

pub fn ask_for_address_loop() -> String {
    loop {
        println!("Please enter your Shoopy Cell Address: ");
        print!("🔋 ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        let address = input.trim().to_string();
        match is_valid_address(&address) {
            Ok(_) => return address,
            Err(e) => println!("❌ {}", e),
        }
    }
}
