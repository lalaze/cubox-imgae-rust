use clap::Parser;
mod cubox;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   user: String,

   #[arg(short, long)]
   password: String,

   #[arg(short, long)]
   folder: String,
}

fn main() {
  let args = Args::parse();
 
  cubox::get_box(args.user, args.password, args.folder)
  // cubox::testtest()
}