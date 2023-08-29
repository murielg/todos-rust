use super::db::Db;
use crate::model;
use sqlb::HasFields;
use sqlx::query;

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub status: TodoStatus,
}

#[derive(sqlb::Fields, Default, Debug, Clone)]
pub struct TodoPatch {
    pub cid: Option<i64>,
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}

sqlb::bindable!(TodoStatus);

// endregion: Todo Types

// region: TodoDao
pub struct TodoDao;

impl TodoDao {
    pub async fn create(db: &Db, data: TodoPatch) -> Result<Todo, model::Error> {
        let mut fields = data.fields();
        fields.push(("cid", 123).into());
        let sql_builder = sqlb::insert()
            .table("todo")
            .data(fields)
            .returning(&["id", "cid", "title", "status"]);
        let todo: Todo = sql_builder.fetch_one(db).await?;
        Ok(todo)
    }

    pub async fn list(db: &Db) -> Result<Vec<Todo>, model::Error> {
        let sb = sqlb::select()
            .table("todo")
            .columns(&["id", "cid", "title", "status"])
            .order_by("!id");
        let todos: Vec<Todo> = sb.fetch_all(db).await?;
        Ok(todos)
    }
}
// endregion: TodoDao

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
