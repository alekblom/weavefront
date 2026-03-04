use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DeployTarget {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub status: TargetStatus,
    pub features: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetStatus {
    Active,
    Planned,
    Experimental,
}

pub fn available_targets() -> Vec<DeployTarget> {
    vec![
        DeployTarget {
            id: "ipfs",
            name: "IPFS",
            description: "InterPlanetary File System — content-addressed, distributed hosting",
            status: TargetStatus::Active,
            features: vec!["static-sites", "spa", "dapp-frontends", "pinning"],
        },
        DeployTarget {
            id: "arweave",
            name: "Arweave",
            description: "Permanent storage — pay once, stored forever",
            status: TargetStatus::Planned,
            features: vec!["permanent-storage", "static-sites", "nft-metadata"],
        },
        DeployTarget {
            id: "filecoin",
            name: "Filecoin",
            description: "Decentralized storage network — large-scale data deals",
            status: TargetStatus::Planned,
            features: vec!["large-files", "storage-deals", "data-archival"],
        },
    ]
}
