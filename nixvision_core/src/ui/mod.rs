use std::io::{self, Write};
use inquire::Text;
pub struct Interface;

impl Interface {
    /// Ahora es estática: se llama con Interface::welcome_banner()
    pub fn welcome_banner() { 
        println!("--------------------------------------------------");
        println!("          🌑 MOON DYNAMICS: NixVision          ");
        println!("              Sistemas de Vision               ");
        println!("--------------------------------------------------");
    }

    pub fn success(msg: &str) {
        println!("\x1b[32m SUCCESS:\x1b[0m {}", msg);
    }

    pub fn error(msg: &str) {
        eprintln!("\x1b[31m ERROR:\x1b[0m {}", msg);
    }

    pub fn info(msg: &str) {
        println!("\x1b[34m INFO:\x1b[0m {}", msg);
    }

    pub fn progress_bar(current: usize, total: usize) {
        let progress = (current as f64 / total as f64) * 20.0;
        print!("\r[");
        for i in 0..20 {
            if (i as f64) < progress { print!("="); }
            else { print!(" "); }
        }
        print!("] {:.2}%", (current as f64 / total as f64) * 100.0);
        io::stdout().flush().unwrap();
    }

    pub fn ask_text(prompt: &str, default: &str) -> String {
        Text::new(prompt)
            .with_default(default)
            .prompt()
            .unwrap_or_else(|_| default.to_string())
    }
}