use structopt_derive::*;

#[derive(StructOpt)]
#[structopt(name = "library_tool")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Command
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "login account", usage = "library-tool login [username] [password]")]
    Login {
        #[structopt(help = "username")]
        username: String,
        #[structopt(help = "password")]
        password: String,
    },
    #[structopt(about = "in/out library", usage = "library-tool library [in/out] [--api/-a]")]
    Library {
        #[structopt(subcommand)]
        op: InOrOut
    },
}

#[derive(StructOpt)]
pub enum InOrOut {
    #[structopt(help = "in", about = "in to library")]
    In {
        #[structopt(help = "api", short = "a", long = "api", default_value = "1")]
        api: u8
    },
    #[structopt(help = "out", about = "out from library")]
    Out
}