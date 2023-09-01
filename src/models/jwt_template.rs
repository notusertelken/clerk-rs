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
pub struct JwtTemplate {
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "claims")]
    pub claims: serde_json::Value,
    #[serde(rename = "lifetime")]
    pub lifetime: i32,
    #[serde(rename = "allowed_clock_skew")]
    pub allowed_clock_skew: i32,
    #[serde(rename = "custom_signing_key", skip_serializing_if = "Option::is_none")]
    pub custom_signing_key: Option<bool>,
    #[serde(rename = "signing_algorithm", skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
    /// Unix timestamp of creation. 
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// Unix timestamp of last update. 
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl JwtTemplate {
    pub fn new(object: Object, id: String, name: String, claims: serde_json::Value, lifetime: i32, allowed_clock_skew: i32, created_at: i64, updated_at: i64) -> JwtTemplate {
        JwtTemplate {
            object,
            id,
            name,
            claims,
            lifetime,
            allowed_clock_skew,
            custom_signing_key: None,
            signing_algorithm: None,
            created_at,
            updated_at,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "jwt_template")]
    JwtTemplate,
}

impl Default for Object {
    fn default() -> Object {
        Self::JwtTemplate
    }
}

