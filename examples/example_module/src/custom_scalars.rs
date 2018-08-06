//! We define the custom scalars present in the GitHub schema. More precise types could be provided here (see tests), as long as they are deserializable.

pub type X509Certificate = String;
pub type URI = String;
pub type HTML = String;
pub type GitTimestamp = String;
pub type GitSSHRemote = String;
pub type GitObjectID = String;
pub type Date = String;
pub type DateTime = String;
