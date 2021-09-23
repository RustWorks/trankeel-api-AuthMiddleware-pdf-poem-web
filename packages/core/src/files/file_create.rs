use async_graphql::InputObject;
use piteo_data::FileType;
use piteo_data::Url;

#[derive(InputObject)]
pub struct FileInput {
    download_url: Url,
    r#type: FileType,
}
