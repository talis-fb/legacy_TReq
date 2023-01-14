#[derive(Clone)]
pub struct ExternalEditor {
    pub editor: String,
}
impl ExternalEditor {
    pub fn setup_and_init() -> Result<Self, String> {
        let editor = std::env::var("EDITOR").map_err(|e| e.to_string())?;
        Ok(Self { editor })
    }
}
