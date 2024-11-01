use clap::Parser;
use directories::UserDirs;
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(
    version,
    long_about = "touchtemplate ('tt') is a way to create a file using a template (if the template is in your templates directory, default ~/Templates)."
)]
struct Config {
    #[arg(help = "The filename of the template.")]
    template: PathBuf,
    #[arg(help = "The name of the file to create.")]
    filename: PathBuf,
}

fn main() {
    let args = Config::parse();
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(template_dir) = user_dirs.template_dir() {
            let template = if args.template.has_root() {
                args.template
            } else {
                let mut t = template_dir.to_path_buf();
                t.push(args.template);
                t
            };
            if template.exists() {
                let _ = fs::copy(template, args.filename);
            } else {
                println!("{template:?} does not exist");
            }
        }
    }
}
