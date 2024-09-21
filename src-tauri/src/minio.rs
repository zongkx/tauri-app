use minio_rsc::client::{BucketArgs, KeyArgs};
use minio_rsc::error::Result as MinioResult;
// Alias to avoid confusion with std::result::Result
use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Deserialize)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Serialize)]
pub struct BucketInfo {
    pub name: String,
}

#[command]
pub async fn list_buckets(config: MinioConfig) -> Result<Vec<BucketInfo>, String> {
    // Initialize MinIO client with credentials
    let provider = StaticProvider::new(&config.access_key, &config.secret_key, None);
    let minio = Minio::builder()
        .endpoint(&config.endpoint)
        .provider(provider)
        .secure(false)  // Set to 'true' if you're using HTTPS, 'false' otherwise
        .build()
        .map_err(|e| format!("Failed to create MinIO client: {}", e))?;  // Proper error handling

    // Retrieve the list of buckets
    let (buckets, _owner) = minio
        .list_buckets()
        .await
        .map_err(|e| format!("Failed to list buckets: {}", e))?;

    // Map the bucket list to a Vec<BucketInfo>
    let buckets_info: Vec<BucketInfo> = buckets
        .into_iter()
        .map(|bucket| BucketInfo { name: bucket.name })
        .collect();

    Ok(buckets_info)
}
