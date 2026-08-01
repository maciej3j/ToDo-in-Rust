//! File: to_do/core/src/main.rs

mod api;
mod enums;
mod structs;
use api::basic_actions::create::create;
use clap::Parser;

use crate::enums::TaskStatus;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    title: String,
    #[arg(short, long)]
    status: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let status_enum = TaskStatus::from_string(&args.status)?;
    let todo_item = create(&args.title, status_enum)?;
    println!("{}", todo_item);
    Ok(())
}
