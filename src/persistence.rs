use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;

use crate::error::{Error, Result};

pub mod models;

pub struct Posts {
    connection: SqliteConnection,
}

impl Posts {
    pub fn new() -> Result<Self> {
        let connection = SqliteConnection::establish(dotenv!("DATABASE_URL"))
            .map_err(Error::DbConnection)?;

        Ok(Self { connection })
    }

    pub fn find<T: AsRef<str>>(&self, link_id: T) -> Option<models::Post> {
        use crate::schema::posts::dsl;

        dsl::posts
            .filter(dsl::link_id.eq(link_id.as_ref()))
            .get_result::<models::Post>(&self.connection).ok()
    }

    pub fn last(&self) -> Option<models::Post> {
        use crate::schema::posts::dsl;

        dsl::posts
            .limit(1)
            .order(dsl::timestamp.desc())
            .get_result::<models::Post>(&self.connection).ok()
    }

    pub fn upsert<T: AsRef<str>>(&self,
                                 link_id: T,
                                 video_link: T,
                                 timestamp: i32) -> Result<()> {
        use crate::schema::posts::dsl;
        println!("Caching {}", link_id.as_ref());
        let db_post_filter = dsl::link_id.eq(link_id.as_ref());

        let update_result = diesel::update(dsl::posts.filter(db_post_filter))
            .set(dsl::timestamp.eq(timestamp))
            .execute(&self.connection);

        match update_result {
            Ok(0) | Err(_) => self.insert(link_id, video_link, timestamp),
            Ok(ok) => Ok(ok),
        }?;

        Ok(())
    }

    fn insert<T: AsRef<str>>(&self,
                             link_id: T,
                             video_link: T,
                             timestamp: i32) -> Result<(usize)> {
        use crate::schema::posts::table;

        println!("inserting {}", timestamp);

        let post = models::NewPost {
            link_id: link_id.as_ref().into(),
            video_link: video_link.as_ref().into(),
            timestamp,
        };

        diesel::insert_into(table)
            .values(post)
            .execute(&self.connection)
            .map_err(Error::DbInsert)
    }
}
