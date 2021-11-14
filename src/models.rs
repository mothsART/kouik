#[derive(Queryable)]
pub struct Program {
    pub id: i32,
    pub keyword: String,
    pub name: String,
    pub locale: String
}
