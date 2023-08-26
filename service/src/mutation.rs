use ::entity::{todo, todo::Entity as Todo};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_todo(
        db: &DbConn,
        form_data: todo::Model,
    ) -> Result<todo::ActiveModel, DbErr> {
        todo::ActiveModel {
            text: Set(form_data.text.to_owned()),
            completed: Set(form_data.completed),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_todo_by_id(
        db: &DbConn,
        id: i32,
        form_data: todo::Model,
    ) -> Result<todo::Model, DbErr> {
        let todo: todo::ActiveModel = Todo::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find todo.".to_owned()))
            .map(Into::into)?;

        todo::ActiveModel {
            id: todo.id,
            text: Set(form_data.text.to_owned()),
            completed: Set(form_data.completed.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_todo(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let todo: todo::ActiveModel = Todo::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find todo.".to_owned()))
            .map(Into::into)?;

        todo.delete(db).await
    }

    pub async fn delete_all_todos(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Todo::delete_many().exec(db).await
    }
}
