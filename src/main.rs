use clap::Parser;
use skyline_generator::*;

#[derive(Parser, Debug)]
#[command(
    name = "skylineg",
    version = "0.0.1",
    about = "A CLI app to generate github skyline"
)]
struct Args {
    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    year: u32,
}

fn main() {
    let args = Args::parse();
    let github_handle = github::GithubContributions::init();

    // github_handle.print_user_id(&args.user);
    let contributions = github_handle
        .get_contributions(&args.user, args.year)
        .unwrap();
    println!("{:?}", contributions.len());
}
