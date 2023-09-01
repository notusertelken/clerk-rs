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
pub struct MergeOrganizationMetadataRequest {
    /// Metadata saved on the organization, that is visible to both your frontend and backend. The new object will be merged with the existing value.
    #[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
    pub public_metadata: Option<serde_json::Value>,
    /// Metadata saved on the organization that is only visible to your backend. The new object will be merged with the existing value.
    #[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
    pub private_metadata: Option<serde_json::Value>,
}

impl MergeOrganizationMetadataRequest {
    pub fn new() -> MergeOrganizationMetadataRequest {
        MergeOrganizationMetadataRequest {
            public_metadata: None,
            private_metadata: None,
        }
    }
}


