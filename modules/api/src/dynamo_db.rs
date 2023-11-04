use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::get_item::{GetItemError, GetItemOutput};
use aws_sdk_dynamodb::operation::scan::{ScanError, ScanOutput};
use aws_sdk_dynamodb::types::AttributeValue;

pub(crate) async fn get_client() -> Client {
    let shared_config = aws_config::load_from_env().await;
    Client::new(&shared_config)
}

pub(crate) async fn get_post(
    client: &Client,
    uid: String,
    updating_date_time: String
) -> Result<GetItemOutput, SdkError<GetItemError>> {
    let result = client
        .get_item()
        .table_name("railson-dev-blog-posts")
        .key("uid", AttributeValue::S(uid.to_string()))
        .key("updating_date_time", AttributeValue::S(updating_date_time.to_string()))
        .send().await;

    return result;
}

pub(crate) async fn get_posts(
    client: &Client,
) -> Result<ScanOutput, SdkError<ScanError>> {
    let result = client
        .scan()
        .table_name("railson-dev-blog-posts")
        .send().await;

    return result;
}