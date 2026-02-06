#[derive(Debug, Deserialize, sqlx::FromRow)]
struct Usuario {
    id: i32,
    nombre: String,
}