use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct LoginRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize, Clone, Debug)]
pub struct RegisterRequest<'a> {
    #[serde(rename = "firstName")]
    pub first_name: &'a str,
    #[serde(rename = "lastName")]
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
