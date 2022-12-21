use std::collections::BTreeMap;
//use k8s_openapi::api::core::v1::Service;
use crate::cli::{interface, interface::Mode};
use crate::core::{watcher, VERSION};

pub async fn initialize() -> anyhow::Result<()> {
    let cli_args = interface::args();

    match cli_args.mode {
        Mode::Watch => {
            /* Keep track of the services */
            let mut service_map = BTreeMap::new();
            watcher::start(&mut service_map).await?;
        }
        Mode::List => {}
        Mode::Version => {
            println!("{}", VERSION);
        }
    }

    Ok(())
}
