use tide::{Server, Request, Response};
use dotenv::dotenv;
use sqlx::{Pool, PgPool, query};
use thiserror::Error;
use serde_json::json;
use tide::http::StatusCode;

#[async_std::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("db url error");
    let db_pool: PgPool = Pool::new(&db_url).await.expect("db error");
    

    let mut app: Server<State> = Server::with_state(State {db_pool});

    app.at("/").get(|req: Request<State>| async move { 
        let db_pool = &req.state().db_pool;
        let row = query!("select 1 as one").fetch_one(db_pool).await?;
        dbg!(row);
        Ok("Hello world") 
    });
    app.listen("127.0.0.1:8080").await.unwrap();
}

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}


#[derive(Error, Debug)]
enum MyError{
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    DbError(#[from] sqlx::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError)
}