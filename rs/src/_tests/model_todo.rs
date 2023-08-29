use crate::model::db::init_db;
use crate::model::todo::{TodoDao, TodoPatch, TodoStatus};

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let data_fx = TodoPatch {
        title: Some("test = model_todo_create 1".to_string()),
        ..Default::default()
    };

    let todo_created = TodoDao::create(&db, data_fx.clone()).await?;

    println!("\n\n ->> {:?}", todo_created);
    assert!(todo_created.id >= 1000, "Id should be >= 1000");
    assert_eq!(data_fx.title.unwrap(), todo_created.title);
    assert_eq!(TodoStatus::Open, todo_created.status);

    Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let todos = TodoDao::list(&db).await?;

    assert_eq!(2, todos.len());
    println!("\n\n ->> {:?}", todos);

    Ok(())
}
