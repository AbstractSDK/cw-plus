use std::env;
use std::sync::Arc;

use abstract_boot::VersionControl;
use abstract_os::{
    objects::module::{ModuleInfo, ModuleVersion},
    objects::module_reference::ModuleReference,
    version_control
};
use boot_core::{
    prelude::*,
    networks::{NetworkInfo, parse_network},
};
use clap::Parser;
use cosmwasm_std::{Addr, Empty};
use log::info;
use semver::Version;
use tokio::runtime::Runtime;

#[boot_contract(Empty, Empty, Empty, Empty)]
pub struct Standalone<Chain>;

// implement chain-generic functions
impl<Chain: BootEnvironment> Standalone<Chain> {
    pub fn new(chain: Chain, id: &str, filepath: &str) -> Self {
        Self(
            Contract::new(id, chain).with_wasm_path(filepath),
        )
    }
}

const CONTRACT_VERSION: &str = "1.0.1";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn deploy(network: NetworkInfo) -> anyhow::Result<()> {
    let mut wasm_files: Vec<std::path::PathBuf> = Vec::new();
    let artifacts_dir = env::var("ARTIFACTS_DIR").unwrap();
    for element in std::path::Path::new(&artifacts_dir).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "wasm" {
                wasm_files.push(path);
            }
        }
    }

    info!("Found {} contracts: {:?} in {}", wasm_files.len(), wasm_files, artifacts_dir);

    let cw_version: Version = CONTRACT_VERSION.parse().unwrap();
    info!("here");

    let rt = Arc::new(Runtime::new()?);
    let options = DaemonOptionsBuilder::default().network(network).build();
    let (_sender, chain) = instantiate_daemon_env(&rt, options?)?;

    let mut version_control = VersionControl::load(
        chain.clone(),
        &Addr::unchecked(std::env::var("VERSION_CONTROL").expect("VERSION_CONTROL not set")),
    );
    info!("here");

    let mut standalones: Vec<Standalone<_>> = wasm_files
        .iter()
        .map(|path| {
            let filename = path.file_name().unwrap().to_str().unwrap();

            let contract_name = filename.split('.').next().unwrap();
            let contract_name = contract_name.replace("_", "-");
            let contract_id = format!("cw-plus:{}", contract_name);
            info!("{}", contract_id);

            let filepath = path.to_str().unwrap();

            Standalone::new(chain.clone(), &contract_id, filepath)
        })
        .collect();

    for standalone in standalones.iter_mut() {
        standalone.upload()?;
    }

    let standalone_instances = standalones.iter().map(|c| c.as_instance()).collect();

    version_control.register_standalones(standalone_instances, &cw_version)?;

    Ok(())
}

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network Id to deploy on
    #[arg(short, long)]
    network_id: String,
}


fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;


    let args = Arguments::parse();

    let network = parse_network(&args.network_id);

    if let Err(ref err) = deploy(network) {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
