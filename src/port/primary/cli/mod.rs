use clap::App;
use voca_rs::*;

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum Protocol {
        Serial,
        Rest,
    }
}

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum Airframe {
        Crj700,
    }
}

#[derive(Debug)]
pub struct CliOpts {
    airframe: Airframe,
    protocol: Protocol,
}

pub struct Cli {}

impl Cli {
    pub fn new() -> CliOpts {
        let matches = App::new("Rusty FMC")
            .version("0.0.1")
            .author("Carl-Erik Bergström <callebstrom@gmail.com>")
            .about("Installs firmware for }Arduino Mega and acts as a bridge between serial or rest and simconnect")
            .arg("-a, --airframe=[AIRFAME] 'Sets what airframe to target'")
            .arg("-p, --protocol=[rest,serial] 'Sets what protocol to use'")
            .subcommand(
                App::new("patch")
                    .about("Installs the required firmware on connected arduino")
            )
            .subcommand(
                App::new("bridge")
                    .about("Run local bridge that translates arduino messages to simconnect")
            )
            .get_matches();

        CliOpts {
            airframe: match matches.value_of("airframe") {
                Some(airframe) => case::capitalize(airframe, true).parse().unwrap(),
                None => Airframe::Crj700,
            },
            protocol: match matches.value_of("protocol") {
                Some(protocol) => case::capitalize(protocol, true).parse().unwrap(),
                None => Protocol::Serial,
            },
        }
    }
}
