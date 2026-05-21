pub mod tcp;
pub mod encode;
pub mod http;
pub mod recon;
pub mod offensive;

pub struct Tool {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub entry_point: fn(args: Vec<String>) -> anyhow::Result<()>,
}