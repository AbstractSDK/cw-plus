use std::env;
use boot_core::prelude::*;
use cosmwasm_std::{Addr, Empty};
use std::sync::Arc;

use boot_core::networks::UNI_5;
use boot_core::prelude::*;

use semver::Version;
use tokio::runtime::Runtime;

use abstract_boot::{VersionControl};
use abstract_os::objects::module::{ModuleInfo, ModuleVersion};
use abstract_os::objects::module_reference::ModuleReference;
use abstract_os::version_control;
use boot_core::networks::terra::PISCO_1;

#[boot_contract(Empty, Empty, Empty, Empty)]
pub struct Standalone<Chain>;

// implement chain-generic functions
impl<Chain: BootEnvironment> Standalone<Chain> {
    pub fn new(chain: Chain, id: &str, filename: &str) -> Self {
        Self(
            Contract::new(id, chain).with_wasm_path(filename),
        )
    }
}

pub fn register_standalone<Chain: BootEnvironment>(
    vc: &VersionControl<Chain>,
    modules: Vec<&Contract<Chain>>,
    version: &Version,
) -> Result<(), BootError> where
    TxResponse<Chain>: IndexResponse {
    let apps_to_register: Result<Vec<(ModuleInfo, ModuleReference)>, BootError> = modules
        .iter()
        .map(|app| {
            Ok((
                ModuleInfo::from_id(&app.id, ModuleVersion::Version(version.to_string()))?,
                ModuleReference::Standalone(app.code_id()?),
            ))
        })
        .collect();
    vc.execute(
        &version_control::ExecuteMsg::AddModules {
            modules: apps_to_register?,
        },
        None,
    )?;
    Ok(())
}

const CONTRACT_VERSION: &str = "1.0.1";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn deploy() -> anyhow::Result<()> {
    let mut faxvec: Vec<std::path::PathBuf> = Vec::new();
    let artifacts_dir = env::var("ARTIFACTS_DIR").unwrap();
    for element in std::path::Path::new(&artifacts_dir).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "wasm" {
                faxvec.push(path);
            }
        }
    }

    // panic!("Found {} contracts: {:?} {}", faxvec.len(), faxvec, artifacts_dir);

    let cw_version: Version = CONTRACT_VERSION.parse().unwrap();
    let network = PISCO_1;

    let rt = Arc::new(Runtime::new()?);
    let options = DaemonOptionsBuilder::default().network(network).build();
    let (_sender, chain) = instantiate_daemon_env(&rt, options?)?;

    let mut version_control = VersionControl::load(
        chain.clone(),
        &Addr::unchecked(std::env::var("VERSION_CONTROL").expect("VERSION_CONTROL not set")),
    );

    let mut standalones: Vec<Standalone<_>> = faxvec
        .iter()
        .map(|path| {
            let filename = path.file_name().unwrap().to_str().unwrap();

            let contract_name = filename.split('.').next().unwrap();
            Standalone::new(chain.clone(), &format!("cw_plus:{}", contract_name), path.to_str().unwrap())
        })
        .collect();

    for standalone in standalones.iter_mut() {
        standalone.upload()?;
    }

    let standalone_instances = standalones.iter().map(|c| c.as_instance()).collect();

    register_standalone(&version_control, standalone_instances, &cw_version)?;

    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = deploy() {
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
