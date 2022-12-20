use nukleus::core::parser;
// use calculator::Compile;
// cfg_if! {
//     if #[cfg(feature = "default")] {
//         use calculator::Jit as Engine;
//     }
//     else {
//         use calculator::Interpreter as Engine;
//     }
// }


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file was provided");
        std::process::exit(-1);
    }
    println!(
        "{:?}",
        parser::parse(&args[1])
    );
}
