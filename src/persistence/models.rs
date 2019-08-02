use crate::schema::posts;

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub link_id: String,
    pub video_link: String,
    pub timestamp: i32,
}

#[derive(Debug, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub link_id: String,
    pub video_link: String,
    pub timestamp: i32,
}
