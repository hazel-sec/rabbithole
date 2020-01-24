use serde::Deserialize;
use std::collections::HashMap;

type Relays = Vec<Relay>;
type GenericResult<T> = Result<T, failure::Error>;

/// API address
const ONIONOO_DETAIL: &str = "https://onionoo.torproject.org/details";

#[derive(Deserialize)]
struct OnionOODetail {
    relays: Vec<Relay>,
}

/// tor relay node information()
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Relay {
    pub measured: bool,
    pub recommended_version: bool,
    pub as_name: Option<String>,
    #[serde(rename = "as")]
    pub AS: Option<String>,
    pub city_name: Option<String>,
    pub country: Option<String>,
    pub country_name: Option<String>,
    pub region_name: Option<String>,
    pub running: bool,
    pub platform: String,
    pub version: String,
    pub fingerprint: String,
    pub exit_policy: Vec<String>,
    pub exit_policy_summary: HashMap<String, Vec<String>>,
    pub nickname: String,
    pub flags: Vec<String>,
    pub or_addresses: Vec<String>,
}

impl Relay {
    /// is entry node?
    pub fn is_entry(&self) -> bool {
        self.flags.iter().any(|x| x.as_str() == "Guard")
    }
    /// is exit node?
    pub fn is_exit(&self) -> bool {
        self.flags.iter().any(|x| x.as_str() == "Exit")
    }
    /// get a or address like "111.22.33.44:9001"
    pub fn or_address(&self) -> Option<String> {
        Some(self.or_addresses.get(0)?.to_owned())
    }
}

/// get tor relay nodes from https://onionoo.torproject.org/details
pub async fn get_tor_relays() -> GenericResult<Relays> {
    let result: OnionOODetail = reqwest::get(ONIONOO_DETAIL)
        .await?
        .json::<OnionOODetail>()
        .await?;
    Ok(result.relays)
}

/// get entry nodes from https://onionoo.torproject.org/details
/// detect with "Guard" flag
pub async fn get_entry_nodes() -> GenericResult<Relays> {
    let nodes = get_tor_relays().await?;
    Ok(nodes.into_iter().filter(|x| x.is_entry()).collect())
}

/// get exit nodes from https://onionoo.torproject.org/details
/// detect with "Exit" flag
pub async fn get_exit_nodes() -> GenericResult<Relays> {
    let nodes = get_tor_relays().await?;
    Ok(nodes.into_iter().filter(|x| x.is_exit()).collect())
}
