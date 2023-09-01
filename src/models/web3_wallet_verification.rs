/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Web3WalletVerification {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "strategy")]
    pub strategy: Strategy,
    #[serde(rename = "nonce")]
    pub nonce: Nonce,
    #[serde(rename = "attempts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Option<i32>>,
    #[serde(rename = "expire_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expire_at: Option<Option<i32>>,
}

impl Web3WalletVerification {
    pub fn new(status: Status, strategy: Strategy, nonce: Nonce) -> Web3WalletVerification {
        Web3WalletVerification {
            status,
            strategy,
            nonce,
            attempts: None,
            expire_at: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "verified")]
    Verified,
}

impl Default for Status {
    fn default() -> Status {
        Self::Verified
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "admin")]
    Admin,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::Admin
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Nonce {
    #[serde(rename = "nonce")]
    Nonce,
}

impl Default for Nonce {
    fn default() -> Nonce {
        Self::Nonce
    }
}

