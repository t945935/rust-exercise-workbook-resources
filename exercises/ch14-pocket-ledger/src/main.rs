fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    if let Err(error) = pocket_ledger::run(&args, &mut output) {
        let stderr = std::io::stderr();
        let mut errors = stderr.lock();
        let _ = pocket_ledger::write_error(&mut errors, &error);
        std::process::exit(2);
    }
}
