use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args); 
    println!("HERE!"); 
}
