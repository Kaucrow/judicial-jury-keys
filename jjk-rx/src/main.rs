use jjk_rx::db::db_component::Db;
use jjk_rx::db::model::Usuario;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = Db::connect("postgres://user:pass@localhost:5432/mi_db", 5).await?;

    // fetch_one
    let usuario = db
        .fetch_one(db.query_as::<Usuario>("SELECT id, nombre FROM usuarios WHERE id = $1").bind(1))
        .await?;
    println!("Usuario: {:?}", usuario);

    // fetch_many
    let usuarios = db
        .fetch_many(db.query_as::<Usuario>("SELECT id, nombre FROM usuarios"))
        .await?;
    println!("Usuarios: {:?}", usuarios);

    // execute (insert/update/delete)
    let filas = db
        .execute(
            db.query("INSERT INTO usuarios (nombre) VALUES ($1)")
                .bind("Ana"),
        )
        .await?;
    println!("Filas afectadas: {}", filas);

    Ok(())
}
