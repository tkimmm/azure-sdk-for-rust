use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account connection string from environment variables.
    let connection_string =
        std::env::var("CONNECTION_STRING").expect("Set env variable CONNECTION_STRING first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let storage_account =
        StorageAccountClient::new_connection_string(http_client.clone(), &connection_string)?
            .as_storage_client();
    let container = storage_account.as_container_client(&container_name);

    let iv = storage_account.list_containers().execute().await?;

    // this is obsolete and will be removed when the blob
    // functions have been migrated.
    let client = client::from_connection_string(&connection_string)?;

    if iv
        .incomplete_vector
        .iter()
        .find(|item| item.name == container_name)
        .is_some()
    {
        panic!("The specified container must not exists!");
    }

    // create the container
    container
        .create()
        .with_public_access(PublicAccess::None)
        .with_timeout(Duration::from_secs(100).into())
        .execute()
        .await?;
    println!("Container {} created", container_name);

    // create 10 blobs
    for i in 0..10u8 {
        client
            .put_block_blob()
            .with_container_name(&container_name)
            .with_blob_name(&format!("blob{}.txt", i))
            .with_content_type("text/plain")
            .with_body("somedata".as_bytes())
            .finalize()
            .await?;
        println!("\tAdded blob {}", i);
    }

    let max_results = NonZeroU32::new(3).unwrap().into();
    let iv = container
        .list_blobs()
        .with_max_results(max_results)
        .execute()
        .await?;

    println!("List blob returned {} blobs.", iv.incomplete_vector.len());
    for cont in iv.incomplete_vector.iter() {
        println!("\t{}\t{} bytes", cont.name, cont.content_length);
    }

    let mut stream = Box::pin(
        container
            .list_blobs()
            .with_max_results(max_results)
            .stream(),
    );

    let mut cnt = 0;
    while let Some(value) = stream.next().await {
        let len = value?.incomplete_vector.len();
        println!("received {} blobs", len);
        match cnt {
            0 | 1 | 2 => assert_eq!(len, 3),
            3 => assert_eq!(len, 1),
            _ => panic!("more than 10 entries??"),
        }
        cnt += 1;
    }

    container.delete().execute().await?;
    println!("Container {} deleted", container_name);

    Ok(())
}
