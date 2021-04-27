use super::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: i16,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub id: i64,
    pub title: &'a str,
    pub body: &'a str,
}
