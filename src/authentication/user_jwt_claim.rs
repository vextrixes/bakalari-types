use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserJwtClaim {
    #[serde(rename = "Bakalari.AccountType")]
    pub account_type: String,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/nameidentifier")]
    pub xmlnameidentifier: Option<String>,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name")]
    pub xmlname: Option<String>,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/upn")]
    pub xmlupn: Option<String>,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/gender")]
    pub xmlgender: Option<String>,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/dateofbirth")]
    pub xmldateofbirth: Option<String>,
    #[serde(rename = "Bakalari.UserType")]
    pub user_type: String,
    #[serde(rename = "Bakalari.SessionId")]
    pub session_id: String,
    #[serde(rename = "Bakalari.TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "Bakalari.PersonIdWithType")]
    pub person_id_with_type: String,
    #[serde(rename = "Bakalari.EvidNumber")]
    pub evid_number: String,
    #[serde(rename = "Bakalari.MachineName")]
    pub machine_name: String,
    pub sub: String,
    pub name: String,
    pub oi_prst: String,
    pub oi_au_id: String,
    pub client_id: String,
    pub oi_tkn_id: String,
    pub aud: String,
    pub scope: String,
    pub jti: String,
    pub exp: i64,
    pub iss: String,
    pub iat: i64,
}
