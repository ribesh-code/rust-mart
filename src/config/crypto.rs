#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>,
}

impl CryptoService {
    pub async fn hash_password(&self, password: String) -> Result<String> {}
}
