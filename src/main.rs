use std::process::exit;

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

    #[arg(short, long, default_value = None)]
    repo: Option<String>,

    #[arg(short, long, default_value = None)]
    owner: Option<String>,
}

fn main() {
    let args = Args::parse();
    let github_handle = github::GithubContributions::init();

    // github_handle.print_user_id(&args.user);
    let contributions = if args.repo.is_none() {
        github_handle
            .get_contributions(&args.user, args.year)
            .unwrap()
    } else {
        if args.owner.is_none() {
            eprintln!("Missing Owner field");
            exit(1);
        };
        github_handle
            .get_contributions_by_repo(
                &args.user,
                &args.owner.unwrap(),
                &args.repo.unwrap(),
                args.year,
            )
            .unwrap()
    };
    stl::create_3d_model(&args.user, args.year, contributions).unwrap();
}
