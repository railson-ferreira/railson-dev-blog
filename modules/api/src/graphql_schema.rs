use juniper::{RootNode, EmptyMutation, EmptySubscription};
use crate::dynamo_db::{get_client, get_post, get_posts};
use chrono::{DateTime};

pub struct QueryRoot;

pub(crate) struct Post {
    pub(crate) uid: String,
    pub(crate) timestamp: i32,
    pub(crate) git_revision: String,
    pub(crate) file_path: String,
    pub(crate) content_markdown: String,
}

#[juniper::graphql_object(description = "There is a message here")]
impl Post {
    pub fn uid(&self) -> String {
        self.uid.clone()
    }
    pub fn timestamp(&self) -> i32 {
        self.timestamp.clone()
    }
    pub fn git_revision(&self) -> String {
        self.git_revision.clone()
    }
    pub fn file_path(&self) -> String {
        self.file_path.clone()
    }
    pub fn content_markdown(&self) -> String {
        self.content_markdown.clone()
    }
}


#[juniper::graphql_object]
impl QueryRoot {
    async fn post(uid: String, updating_date_time: String) -> Option<Post> {
        let client = get_client().await;
        let result = get_post(&client, uid, updating_date_time).await;

        let item = match result.unwrap().item {
            Some(item) => item,
            None => return None
        };

        let uid = item.get("uid").unwrap().as_s().unwrap().to_string();
        let timestamp = DateTime::parse_from_rfc3339(item.get("updating_date_time").unwrap().as_s().unwrap().as_str()).unwrap().timestamp() as i32;
        let git_revision = item.get("git_revision").unwrap().as_s().unwrap().to_string();
        let file_path = item.get("file_path").unwrap().as_s().unwrap().to_string();
        let content_markdown = item.get("content_markdown").unwrap().as_s().unwrap().to_string();

        return Some(
            Post {
                uid: uid,
                timestamp: timestamp,
                git_revision: git_revision,
                file_path: file_path,
                content_markdown: content_markdown,
            }
        );
    }

    async fn posts() -> Vec<Post> {
        let client = get_client().await;
        let result = get_posts(&client).await;

        let items = result
            .unwrap()
            .items
            .unwrap();

        let post_list = items.iter()
            .map(|item| {
                let uid = item.get("uid").unwrap().as_s().unwrap().to_string();
                let timestamp = DateTime::parse_from_rfc3339(item.get("updating_date_time").unwrap().as_s().unwrap().as_str()).unwrap().timestamp() as i32;
                let git_revision = item.get("git_revision").unwrap().as_s().unwrap().to_string();
                let file_path = item.get("file_path").unwrap().as_s().unwrap().to_string();
                let content_markdown = item.get("content_markdown").unwrap().as_s().unwrap().to_string();

                return Post {
                    uid: uid,
                    timestamp: timestamp,
                    git_revision: git_revision,
                    file_path: file_path,
                    content_markdown: content_markdown,
                };
            })
            .collect();

        return post_list;
    }
}


// A root schema consists of a query and a mutation.
// Request queries can be executed against a RootNode.
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    return Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new());
}
