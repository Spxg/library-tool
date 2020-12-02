use structopt_derive::*;

#[derive(StructOpt)]
#[structopt(name = "library_tool")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Command
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(help = "login account")]
    Login {
        #[structopt(help = "username")]
        username: String,
        #[structopt(help = "password")]
        password: String,
    },
    #[structopt(help = "in/out library")]
    Library {
        #[structopt(help = "in", short = "i", long = "in", default_value = "1")]
        // 'in' is a keyword, so.......
        into: u8,
        #[structopt(help = "out", short = "o", long = "out")]
        out: bool,
    },
}