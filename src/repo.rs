use crate::models::TodoList;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(db: &Client) -> Result<Vec<TodoList>, io::Error> {
    let stmt = db.prepare("SELECT * FROM todo_list").await.unwrap();
    let todos = db
        .query(&stmt, &[])
        .await
        .expect("error getting todo")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}
