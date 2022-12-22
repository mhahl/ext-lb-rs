pub(super) mod engine;
pub(super) mod templates;
pub(super) mod watcher;

pub static VERSION: &'static str = "0.0.1";
pub static DEFAULT_LABEL: &'static str = "hahl.au/ingress";
pub static HAPROXY_CONFIG: &'static str = "/etc/haproxy/haproxy.cfg";
