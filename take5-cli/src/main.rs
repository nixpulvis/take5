//! Running a game of take5 from the command line.
//!
//! # Usage
//!
//! ```sh
//! The card game take5 (or 6nimmit!).
//!
//! Usage:
//!     take5 <players>
//!     take5 [--conf=<file>, --bulls=<file>, --human] <players>
//!     take5 [-hv]
//!
//! Options:
//!     -h, --help      Show this screen.
//!     -v, --version   Show the version of take5.
//!     --conf=<file>   Global configurations for the game.
//!     --bulls=<file>  Designates file as the configuration for bull values.
//!                     This file should contian exactly `deck_size` comma separated bull values.
//!     -H, --human     Use human players instead of AI players.
//! ```

extern crate docopt;
extern crate serde;
extern crate take5;
#[cfg(feature = "ext")]
extern crate take5_ext;

use args::Args;
use take5::{Configuration, Game};

fn main() {
    let args = Args::parse();

    if let Some(ref f) = args.flag_conf {
        unsafe {
            Configuration::load(f);
        }
    }

    let mut game = Game::new(init::dealer(&args), init::players(&args));
    game.run();
    game.report();
}

#[cfg(feature = "ext")]
mod init {
    use args::Args;
    use take5::dealer::{Dealer, StandardDealer};
    use take5::player::{AiPlayer, Player};
    use take5_ext::{CustomDealer, StdinPlayer};

    pub fn dealer(args: &Args) -> Box<dyn Dealer> {
        match args.flag_bulls {
            None => Box::new(StandardDealer::default()) as Box<dyn Dealer>,
            Some(ref s) => Box::new(CustomDealer::new(s)) as Box<dyn Dealer>,
        }
    }

    pub fn players(args: &Args) -> Vec<Box<dyn Player>> {
        (0..args.arg_players)
            .map(|id| {
                if args.flag_human {
                    Box::new(StdinPlayer::new(id)) as Box<dyn Player>
                } else {
                    Box::new(AiPlayer::new(id)) as Box<dyn Player>
                }
            })
            .collect()
    }
}

#[cfg(not(feature = "ext"))]
mod init {
    use args::Args;
    use take5::dealer::{Dealer, StandardDealer};
    use take5::player::{AiPlayer, Player};

    pub fn dealer(_: &Args) -> Box<Dealer> {
        Box::new(StandardDealer::default()) as Box<Dealer>
    }

    pub fn players(args: &Args) -> Vec<Box<Player>> {
        (0..args.arg_players)
            .map(|id| Box::new(AiPlayer::new(id)) as Box<Player>)
            .collect()
    }
}

mod args;
