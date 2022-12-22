use crate::core::templates::{render_templates, TemplateData};
use crate::core::{DEFAULT_LABEL, HAPROXY_CONFIG};
use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::core::v1::{Node, Service};
use kube::{api::ListParams, runtime::{watcher, watcher::Event, WatchStreamExt}, Api, Client, Resource, api};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

/**
 * List and watch services.
 * @param map {BTreeMap} keep service state.
 */
pub(crate) async fn start(map: &mut BTreeMap<String, Service>) -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let events: Api<Service> = Api::all(client);
    let params = ListParams::default().timeout(60).labels(DEFAULT_LABEL);

    let ew = watcher(events, params);

    println!("Watching for {}", DEFAULT_LABEL);

    pin_mut!(ew);
    while let Some(service) = ew.try_next().await? {
        handle_service(client.clone(), service, map).await?;
    }

    Ok(())
}

/**
 * Return a list of nodes with the `DEFAULT_LABEL` applied.
 */
async fn get_backend_nodes() -> anyhow::Result<Vec<Node>> {
    let client = Client::try_default().await?;
    let nodes: Api<Node> = Api::all(client);
    let params = ListParams::default().timeout(60).labels(DEFAULT_LABEL);
    let node_list = nodes.list(&params).await?;

    Ok(node_list.items)
}

fn is_nodeport(service: &Service) -> bool {
    service.spec.as_ref().unwrap().type_.as_ref().unwrap() == "NodePort"
}

/**
 * Return the uuid of the service.
 */
fn get_service_uuid(service: Service) -> anyhow::Result<(String)> {
    let s = service.clone();
    let uuid = s.metadata.uid.unwrap();
    Ok(uuid)
}

/**
 * Make the config good.
 */
async fn reconcile(map: &BTreeMap<String, Service>) -> anyhow::Result<()> {

    /* Template data */
    let template_data = TemplateData {
        services: map.clone(),
        nodes: get_backend_nodes().await?,
    };

    /* Render out the template string */
    let template = render_templates()?;
    let haproxy_config = template.render("haproxy", &template_data)?;

    /* Write the config to disk */
    let mut haproxy_file = std::fs::File::create(HAPROXY_CONFIG)?;
    haproxy_file.write_all(haproxy_config.as_bytes())?;

    Ok(())
}


/**
 * Handle the service event.
 */
async fn handle_service(
    client: Client,
    event: Event<Service>,
    map: &mut BTreeMap<String, Service>,
) -> anyhow::Result<()> {
    println!("{:?}", event);

    match event {
        /*
         * When the service is applied we insert the service into
         * `map` which we then build the haproxy config from.
         */
        Event::Applied(s) => {
            if is_nodeport(&s) {
                let uuid = get_service_uuid(s.clone())?;
                map.insert(uuid, s.clone());
            }
        }

        /*
         * When the service is applied we insert the service into
         * `map` which we then build the haproxy config from.
         */
        Event::Deleted(s) => {
            if is_nodeport(&s) {
                let uuid = get_service_uuid(s.clone())?;
                map.remove(&uuid);
            }
        }

        /*
         * Watch stream was restart.
         * Rebuild the whole list.
         */
        Event::Restarted(services) => {
            map.clear();
            for s in services {
                if is_nodeport(&s) {
                    let uuid = get_service_uuid(s.clone())?;
                    map.insert(uuid, s.clone());
                }
            }
        }
    }

    reconcile(map).await
}
