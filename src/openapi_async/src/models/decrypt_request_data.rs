/*
 * NetHSM
 *
 * All endpoints expect exactly the specified JSON. Additional properties will cause a Bad Request Error (400). All HTTP errors contain a JSON structure with an explanation of type string. All <a href=\"https://tools.ietf.org/html/rfc4648#section-4\">base64</a> encoded values are Big Endian.
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DecryptRequestData {
    #[serde(rename = "mode")]
    pub mode: crate::models::DecryptMode,
    #[serde(rename = "encrypted")]
    pub encrypted: String,
    #[serde(rename = "iv", skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
}

impl DecryptRequestData {
    pub fn new(mode: crate::models::DecryptMode, encrypted: String) -> DecryptRequestData {
        DecryptRequestData {
            mode,
            encrypted,
            iv: None,
        }
    }
}


