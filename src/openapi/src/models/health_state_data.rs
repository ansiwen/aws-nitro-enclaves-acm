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
pub struct HealthStateData {
    #[serde(rename = "state")]
    pub state: crate::models::SystemState,
}

impl HealthStateData {
    pub fn new(state: crate::models::SystemState) -> HealthStateData {
        HealthStateData {
            state,
        }
    }
}


