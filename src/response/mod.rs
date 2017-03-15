pub trait Item {}

#[derive(Debug, RustcDecodable)]
pub struct User {
    pub id: i32,
    pub login: String,
}

impl Item for User {}


#[derive(Debug, RustcDecodable)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub owner: User,
}

impl Item for Repository {}

#[derive(Debug, RustcDecodable)]
pub struct SearchResponse<T: Item> {
    pub total_count: i32,
    pub items: Vec<T>,
}
