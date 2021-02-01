extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::Region;
use rusoto_s3::{ListObjectsV2Output, ListObjectsV2Request, S3Client, S3};

async fn list() -> ListObjectsV2Output {
    let client = S3Client::new(Region::UsEast1);
    let bucket: String = format!("{}", "hyiromori-files");
    let prefix: Option<String> = Some(format!("{}", "archive/"));
    let request = ListObjectsV2Request {
        bucket,
        prefix,
        continuation_token: None,
        delimiter: None,
        encoding_type: None,
        expected_bucket_owner: None,
        fetch_owner: None,
        max_keys: None,
        request_payer: None,
        start_after: None,
    };
    return client.list_objects_v2(request).await.unwrap();
}

fn main() {
    let result: ListObjectsV2Output = list();
}
