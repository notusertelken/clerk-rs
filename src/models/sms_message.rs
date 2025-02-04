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
pub struct SmsMessage {
	#[serde(rename = "object")]
	pub object: Object,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(
		rename = "slug",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub slug: Option<Option<String>>,
	#[serde(rename = "from_phone_number")]
	pub from_phone_number: String,
	#[serde(rename = "to_phone_number")]
	pub to_phone_number: String,
	#[serde(rename = "phone_number_id", deserialize_with = "Option::deserialize")]
	pub phone_number_id: Option<String>,
	#[serde(
		rename = "user_id",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub user_id: Option<Option<String>>,
	#[serde(rename = "message")]
	pub message: String,
	#[serde(rename = "status")]
	pub status: String,
	#[serde(
		rename = "data",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub data: Option<Option<serde_json::Value>>,
	#[serde(rename = "delivered_by_clerk")]
	pub delivered_by_clerk: bool,
}

impl SmsMessage {
	pub fn new(
		object: Object,
		id: String,
		from_phone_number: String,
		to_phone_number: String,
		phone_number_id: Option<String>,
		message: String,
		status: String,
		delivered_by_clerk: bool,
	) -> SmsMessage {
		SmsMessage {
			object,
			id,
			slug: None,
			from_phone_number,
			to_phone_number,
			phone_number_id,
			user_id: None,
			message,
			status,
			data: None,
			delivered_by_clerk,
		}
	}
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "sms_message")]
	SmsMessage,
}

impl Default for Object {
	fn default() -> Object {
		Self::SmsMessage
	}
}
