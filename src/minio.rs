use anyhow::Result;
use log::info;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::Region::Custom;

pub struct Client {
    bucket: Bucket,
}

impl Client {
    pub fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Result<Client> {
        let region = Custom {
            endpoint: endpoint.to_string(),
            region: "".to_string(),
        };
        let credential = Credentials {
            access_key: Some(access_key.to_string()),
            secret_key: Some(secret_key.to_string()),
            security_token: None,
            session_token: None,
        };
        let bucket = Bucket::new_with_path_style("default", region, credential)?;
        Ok(Client { bucket })
    }

    pub fn get_object(&self, path: &str) -> Result<Vec<u8>> {
        info!("get object; path {}", path);
        let (result, code) = self.bucket.get_object_blocking(path)?;
        info!("get object; code {}", code);
        Ok(result)
    }

    pub fn put_object(&self, path: &str, data: &[u8]) -> Result<()> {
        info!("put object; path {}", path);
        let (_result, code) = self.bucket.put_object_blocking(path, data)?;
        info!("put object; code {}", code);
        Ok(())
    }
}
