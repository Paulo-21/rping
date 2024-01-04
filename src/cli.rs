#[cfg(feature = "cli")]
use clap::builder::styling::{AnsiColor, Effects, Styles};
#[cfg(feature = "cli")]
use clap::Parser;

#[cfg(feature = "cli")]
fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[cfg(feature = "cli")]
#[derive(Parser, Debug, Clone)]
#[command(
    author = "Mahmoud Harmouch",
    version,
    name = "rping",
    propagate_version = true,
    styles = styles(),
    help_template = r#"{before-help}{name} {version}
{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

AUTHORS:
    {author}
"#,
    about=r#"
 ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄▄▄  ▄▄        ▄  ▄▄▄▄▄▄▄▄▄▄▄ 
▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌▐░░▌      ▐░▌▐░░░░░░░░░░░▌
▐░█▀▀▀▀▀▀▀█░▌▐░█▀▀▀▀▀▀▀█░▌ ▀▀▀▀█░█▀▀▀▀ ▐░▌░▌     ▐░▌▐░█▀▀▀▀▀▀▀▀▀ 
▐░▌       ▐░▌▐░▌       ▐░▌     ▐░▌     ▐░▌▐░▌    ▐░▌▐░▌          
▐░█▄▄▄▄▄▄▄█░▌▐░█▄▄▄▄▄▄▄█░▌     ▐░▌     ▐░▌ ▐░▌   ▐░▌▐░▌ ▄▄▄▄▄▄▄▄ 
▐░░░░░░░░░░░▌▐░░░░░░░░░░░▌     ▐░▌     ▐░▌  ▐░▌  ▐░▌▐░▌▐░░░░░░░░▌
▐░█▀▀▀▀█░█▀▀ ▐░█▀▀▀▀▀▀▀▀▀      ▐░▌     ▐░▌   ▐░▌ ▐░▌▐░▌ ▀▀▀▀▀▀█░▌
▐░▌     ▐░▌  ▐░▌               ▐░▌     ▐░▌    ▐░▌▐░▌▐░▌       ▐░▌
▐░▌      ▐░▌ ▐░▌           ▄▄▄▄█░█▄▄▄▄ ▐░▌     ▐░▐░▌▐░█▄▄▄▄▄▄▄█░▌
▐░▌       ▐░▌▐░▌          ▐░░░░░░░░░░░▌▐░▌      ▐░░▌▐░░░░░░░░░░░▌
 ▀         ▀  ▀            ▀▀▀▀▀▀▀▀▀▀▀  ▀        ▀▀  ▀▀▀▀▀▀▀▀▀▀▀ 

🌊 RPING CLI
============

A powerful command-line tool for executing TCP SYN flooding attacks.
Flood a target with a high volume of SYN packets to overwhelm and
disrupt its network.

FEATURES:
  - Packet Length: Set the length of SYN packets to be sent.
  - Target IP: Specify the target IP address to flood.
  - Target Port: Set the target port number for the attack.
  - Threads: Set the number of threads for the attack.

USAGE:
  rping [OPTIONS]

EXAMPLES:
  Perform SYN flooding attack:
    rping -s 100 -t 127.0.0.1 -p 80 -h 8

For more information, visit: https://github.com/wiseaidev/rping
"#
)]
#[cfg(feature = "cli")]
pub struct Cli {
    #[arg(global = true, short, long)]
    pub verbose: bool,

    /// Target ip address.
    #[arg(short = 't', long = "target")]
    pub target: String,

    /// Target port number.
    #[arg(short = 'p', long = "port", default_value_t = 80)]
    pub port: usize,

    /// Length of SYN packets.
    #[arg(short = 's', long = "size", default_value_t = 1500)]
    pub size: usize,

    /// Number of threads.
    #[arg(short = 'h', long = "threads", default_value_t = 8)]
    pub threads: usize,
}
