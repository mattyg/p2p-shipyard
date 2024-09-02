uniffi::setup_scaffolding!();

#[derive(uniffi::Record)]
pub struct Thing {
    pub name: String,
    pub description: String,
}