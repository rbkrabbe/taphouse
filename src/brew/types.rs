use serde::Deserialize;

/// Deserializes a JSON `null` as `T::default()` instead of failing.
/// Use alongside `#[serde(default)]` so missing keys also get the default.
fn null_as_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Versions {
    pub stable: Option<String>,
    pub head: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct FormulaInfo {
    pub name: String,
    pub full_name: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub homepage: String,
    pub versions: Versions,
    pub license: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub tap: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct CaskInfo {
    pub token: String,
    #[serde(default)]
    pub name: Vec<String>,
    #[serde(default, deserialize_with = "null_as_default")]
    pub desc: String,
    #[serde(default)]
    pub homepage: String,
    #[serde(default)]
    pub version: String,
    pub auto_updates: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageKind {
    Formula,
    Cask,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RemoteData<T> {
    NotLoaded,
    Loading,
    Loaded(T),
    Failed(String),
}
