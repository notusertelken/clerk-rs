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
pub struct UpdateOAuthApplicationRequest {
	/// The new name of the OAuth application
	#[serde(rename = "name")]
	pub name: String,
	/// The new callback URL of the OAuth application
	#[serde(rename = "callback_url")]
	pub callback_url: String,
}

impl UpdateOAuthApplicationRequest {
	pub fn new(name: String, callback_url: String) -> UpdateOAuthApplicationRequest {
		UpdateOAuthApplicationRequest { name, callback_url }
	}
}
