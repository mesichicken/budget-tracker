use clap::{Parser, Subcommand};
#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作る
    New,
    /// 口座に入金する
    Deposit,
    /// 口座から出金する
    Withdraw,
    /// CSVからインポートする
    Import,
    /// レポートを出力する
    Report,
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::New => new(),
        Command::Deposit => unimplemented!(),
        Command::Withdraw => unimplemented!(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}

fn new() {
    println!("New command");
}