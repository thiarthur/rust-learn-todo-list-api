use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SuccessResponse {
    pub success: bool,
}
