#[derive(Queryable)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub department_id: i32,
}
