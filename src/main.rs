use clap::Parser;
use katac::{create_day, get_curday, get_dst, get_src};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    katas_dir: Option<String>,

    #[arg(short, long)]
    days_dir: Option<String>,

    #[arg(required = true, num_args = 1..)]
    kata_names: Vec<String>,
}

pub fn copy_kata(args: Args) {
    let copy_options = fs_extra::dir::CopyOptions::new();
    for kata_name in &args.kata_names {
        print!("Copying {} to day{}...", kata_name, get_curday());
        let src = get_src(kata_name, args.katas_dir.clone());
        let dst = get_dst(args.days_dir.clone());
        fs_extra::copy_items(&[src], dst, &copy_options).unwrap();
    }
}

fn main() {
    let args = Args::parse();

    create_day();
    copy_kata(args);
}
