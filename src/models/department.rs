#[derive(Queryable)]
pub struct Department {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
