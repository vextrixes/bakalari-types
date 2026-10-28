use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginSuccess {
    #[serde(rename = "bak:UserId")]
    pub user_id: String,
    pub access_token: String,
    pub token_type: TokenType,
    pub expires_in: i64,
    pub refresh_token: String,
    #[serde(rename = "bak:ApiVersion")]
    pub api_version: String,
    #[serde(rename = "bak:AppVersion")]
    pub app_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginRequest {
    client_id: String,
    grant_type: String,
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Bearer,
}
