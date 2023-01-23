use crate::{
    database::DATABASE_INSTANCE,
    diesel::ExpressionMethods,
    foc_error::{FocError, Result},
    schema::{self, metadata as metadata_schema},
};
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    prelude::*,
    serialize::{self, Output, ToSql},
    sql_types::{self, Nullable, Text},
    AsExpression, FromSqlRow,
};
use infer::{MatcherType, Type};
use serde::{Deserialize, Serialize};

pub struct OptTextToOptVec(Option<Vec<String>>);
impl From<OptTextToOptVec> for Option<Vec<String>> {
    fn from(s: OptTextToOptVec) -> Self {
        s.0
    }
}
impl<DB> Queryable<Nullable<Text>, DB> for OptTextToOptVec
where
    DB: Backend,
    Option<String>: FromSql<Nullable<Text>, DB>,
{
    type Row = Option<String>;

    fn build(opt_str: Option<String>) -> deserialize::Result<Self> {
        match opt_str {
            Some(s) => {
                if s.is_empty() {
                    Ok(OptTextToOptVec(None))
                } else {
                    Ok(OptTextToOptVec(Some(
                        s.split(';').map(|s| s.to_string()).collect(),
                    )))
                }
            }
            None => Ok(OptTextToOptVec(None)),
        }
    }
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = sql_types::Text)]
pub struct OptVecToOptText(pub Option<String>);
impl From<Option<Vec<String>>> for OptVecToOptText {
    fn from(opt_vec: Option<Vec<String>>) -> Self {
        match opt_vec {
            Some(v) => OptVecToOptText(Some(v.join(";"))),
            None => OptVecToOptText(None),
        }
    }
}
impl From<Vec<String>> for OptVecToOptText {
    fn from(v: Vec<String>) -> Self {
        if v.is_empty() {
            OptVecToOptText(None)
        } else {
            OptVecToOptText(Some(v.join(";")))
        }
    }
}
impl<DB> ToSql<sql_types::Text, DB> for OptVecToOptText
where
    DB: Backend,
    String: ToSql<sql_types::Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        self.0.to_sql(out)
    }
}

pub struct TextToContentType(ContentType);
impl From<TextToContentType> for ContentType {
    fn from(s: TextToContentType) -> Self {
        s.0
    }
}
impl<DB> Queryable<Text, DB> for TextToContentType
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(s: String) -> deserialize::Result<Self> {
        Ok(TextToContentType(ContentType::from_string(&s)?))
    }
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = sql_types::Text)]
pub struct ContentTypeToText(pub String);
impl From<ContentType> for ContentTypeToText {
    fn from(ct: ContentType) -> Self {
        ContentTypeToText(ct.to_string())
    }
}
impl<DB> ToSql<sql_types::Text, DB> for ContentTypeToText
where
    DB: Backend,
    String: ToSql<sql_types::Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        self.0.to_sql(out)
    }
}

pub struct TextToStatus(Status);
impl From<TextToStatus> for Status {
    fn from(s: TextToStatus) -> Self {
        s.0
    }
}
impl<DB> Queryable<Text, DB> for TextToStatus
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(s: String) -> deserialize::Result<Self> {
        Ok(TextToStatus(Status::from_string(&s)?))
    }
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = sql_types::Text)]
pub struct StatusToText(pub String);
impl From<Status> for StatusToText {
    fn from(s: Status) -> Self {
        StatusToText(s.to_string())
    }
}
impl<DB> ToSql<sql_types::Text, DB> for StatusToText
where
    DB: Backend,
    String: ToSql<sql_types::Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        self.0.to_sql(out)
    }
}

#[derive(
    Serialize, Deserialize, Queryable, Identifiable, AsChangeset, Insertable, Default, Clone, Debug,
)]
#[diesel(primary_key(hash))]
#[diesel(table_name = crate::schema::metadata)]
#[diesel(treat_none_as_null = true)]
pub struct Metadata {
    pub hash: String,
    pub name: String,
    pub path: String,
    #[diesel(serialize_as = ContentTypeToText)]
    #[diesel(deserialize_as = TextToContentType)]
    #[diesel(column_name = contentType)]
    pub content_type: ContentType,
    #[diesel(serialize_as = StatusToText)]
    #[diesel(deserialize_as = TextToStatus)]
    pub status: Status,
    #[diesel(column_name = timestampCreated)]
    pub timestamp_created: i32,
    #[diesel(column_name = timestampModified)]
    pub timestamp_modified: i32,
    pub extension: Option<String>,
    #[diesel(deserialize_as = OptTextToOptVec)]
    #[diesel(serialize_as = OptVecToOptText)]
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ContentType {
    Image,
    Audio,
    Video,
    Gif,
    Link,
    Other,
}

impl ContentType {
    pub fn from_kind(kind: &Type) -> Result<Self> {
        match kind.matcher_type() {
            MatcherType::Image => {
                if kind.mime_type() == "image/gif" {
                    Ok(ContentType::Gif)
                } else {
                    Ok(ContentType::Image)
                }
            }
            MatcherType::Video => Ok(ContentType::Video),
            MatcherType::Audio => Ok(ContentType::Audio),
            _ => Err(FocError::Extension("Unsupported file extension".to_owned())),
        }
    }

    pub fn from_string(content_type: &str) -> Result<Self> {
        match content_type {
            "Image" => Ok(ContentType::Image),
            "Gif" => Ok(ContentType::Gif),
            "Video" => Ok(ContentType::Video),
            "Audio" => Ok(ContentType::Audio),
            "Link" => Ok(ContentType::Link),
            "Other" => Ok(ContentType::Other),
            _ => Err(FocError::Extension("Invalid content type".to_owned())),
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for ContentType {
    fn default() -> Self {
        ContentType::Other
    }
}


pub async fn get_all_metadata() -> Result<Vec<Metadata>> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    let results = metadata_schema::table
        .load::<Metadata>(&mut conn)
        .expect("Error loading metadata");

    Ok(results)
}

pub async fn get_metadata_by_hash(hash_to_find: &str) -> Result<Metadata> {
    use schema::metadata::dsl::*;
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    let result = metadata
        .find(hash_to_find)
        .first::<Metadata>(&mut conn)
        .expect("Error loading metadata");

    Ok(result)
}

pub async fn insert_metadata(new_metadata: Metadata) -> Result<()> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    diesel::insert_into(metadata_schema::table)
        .values(new_metadata)
        .execute(&mut conn)?;

    Ok(())
}

pub async fn update_metadata(metadata_to_update: Metadata) -> Result<()> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    let values_to_update = metadata_to_update.clone();
    let update_query = diesel::update(&metadata_to_update);
    update_query.set(values_to_update).execute(&mut conn)?;

    Ok(())
}

pub async fn delete_metadata(hashes_to_delete: &Vec<String>) -> Result<()> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    diesel::delete(metadata_schema::table.filter(metadata_schema::hash.eq_any(hashes_to_delete)))
        .execute(&mut conn)?;

    Ok(())
}
pub async fn get_all_tags() -> Result<Vec<Option<Vec<String>>>> {
    let mut conn = DATABASE_INSTANCE.get().unwrap().get().unwrap();

    let results: Vec<Option<Vec<String>>> = metadata_schema::table
        .select(metadata_schema::tags)
        .load::<OptTextToOptVec>(&mut conn)
        .map(|v| v.into_iter().map(Into::into).collect())
        .expect("Error loading metadata");

    Ok(results)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Valid,
    Deleted,
    Duplicate,
}

impl Default for Status {
    fn default() -> Self {
        Status::Valid
    }
}

impl Status {
    pub fn from_string(status: &str) -> Result<Self> {
        match status {
            "Valid" => Ok(Status::Valid),
            "Deleted" => Ok(Status::Deleted),
            "Duplicate" => Ok(Status::Duplicate),
            _ => Err(FocError::Extension("Invalid status".to_owned())),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
