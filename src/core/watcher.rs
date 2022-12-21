use crate::core::{templates::Action, DEFAULT_LABEL};
use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::core::v1::Service;
use kube::{
    api::ListParams,
    runtime::{watcher, watcher::Event, WatchStreamExt},
    Api, Client, Resource,
};
use std::collections::BTreeMap;

/**
 * List and watch services.
 * @param map {BTreeMap} keep service state.
 */
pub(crate) async fn start(map: &mut BTreeMap<String, Service>) -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let events: Api<Service> = Api::all(client);
    let params = ListParams::default()
        .timeout(60)
        .labels(DEFAULT_LABEL);

    let ew = watcher(events, params);

    println!("Watching for {}", DEFAULT_LABEL);

    pin_mut!(ew);
    while let Some(service) = ew.try_next().await? {
        handle_service(service, map)?;
    }

    Ok(())
}

fn get_service_uuid(service: Service) -> anyhow::Result<(String)> {
    let s = service.clone();
    let uuid = s.metadata.uid.unwrap();
    Ok(uuid)
}

fn reconcile( map: &BTreeMap<String, Service>) -> anyhow::Result<()> {

    println!("{:?}", map);


    Ok(())
}

/**
 * Handle the service event.
 */
fn handle_service(
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
            let uuid = get_service_uuid(s.clone())?;
            map.insert(uuid, s.clone());
        }

        /*
         * When the service is applied we insert the service into
         * `map` which we then build the haproxy config from.
         */
        Event::Deleted(s) => {
            let uuid = get_service_uuid(s.clone())?;
            map.remove(uuid.as_str());
        }

        /*
         * Watch stream was restart.
         * Rebuild the whole list.
         */
        Event::Restarted(services) => {
            map.clear();
            for s in services {
                let uuid = get_service_uuid(s.clone())?;
                map.insert(uuid, s.clone());
            }
        }
    }

    reconcile(map)

}
