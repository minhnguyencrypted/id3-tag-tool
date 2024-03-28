use clap::Parser;

#[derive(Parser)]
#[command(version, arg_required_else_help(true))]
struct Args {
    /// ID of the Beatport track
    #[arg(long, required(false))] 
    id: String,

    /// Save track info to a file
    #[arg(short('o'), long("track"), required(false))]
    track_file: String,

    /// Download track artwork to a file
    #[arg(long("artwork"), required(false))]
    artwork_file: String,
}

fn main() {
    let args = Args::parse();
}
