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
pub struct SamlAccount {
    #[serde(rename = "id")]
    pub id: String,
    /// String representing the object's type. Objects of the same type share the same value. 
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "email_address")]
    pub email_address: String,
    #[serde(rename = "first_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Option<String>>,
    #[serde(rename = "last_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Option<String>>,
    #[serde(rename = "provider_user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub provider_user_id: Option<Option<String>>,
    #[serde(rename = "verification", deserialize_with = "Option::deserialize")]
    pub verification: Option<Box<crate::models::SamlAccountVerification>>,
}

impl SamlAccount {
    pub fn new(id: String, object: Object, provider: String, active: bool, email_address: String, verification: Option<crate::models::SamlAccountVerification>) -> SamlAccount {
        SamlAccount {
            id,
            object,
            provider,
            active,
            email_address,
            first_name: None,
            last_name: None,
            provider_user_id: None,
            verification: if let Some(x) = verification {Some(Box::new(x))} else {None},
        }
    }
}

/// String representing the object's type. Objects of the same type share the same value. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "saml_account")]
    SamlAccount,
}

impl Default for Object {
    fn default() -> Object {
        Self::SamlAccount
    }
}

