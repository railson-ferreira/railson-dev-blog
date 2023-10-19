use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::put_item::{PutItemError, PutItemOutput};
use aws_sdk_dynamodb::types::AttributeValue;

pub(crate) async fn get_client() -> Client {
    let shared_config = aws_config::load_from_env().await;
    Client::new(&shared_config)
}

pub(crate) async fn insert_post(
    client: &Client,
    uid: String,
    updating_date_time: String,
    git_revision: String,
    file_path: String,
    content_markdown: String,
) -> Result<PutItemOutput, SdkError<PutItemError>> {
    let result = client
        .put_item()
        .table_name("railson-dev-blog-posts")
        .item("uid", AttributeValue::S(uid))
        .item("updating_date_time", AttributeValue::S(updating_date_time))
        .item("git_revision", AttributeValue::S(git_revision))
        .item("file_path", AttributeValue::S(file_path))
        .item("content_markdown", AttributeValue::S(content_markdown))
        .send().await;

    return result;
}