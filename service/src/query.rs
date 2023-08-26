use ::entity::{todo, todo::Entity as Todo};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_todo_by_id(db: &DbConn, id: i32) -> Result<Option<todo::Model>, DbErr> {
        Todo::find_by_id(id).one(db).await
    }

    pub async fn find_all_todos(db: &DbConn) -> Result<Vec<todo::Model>, DbErr> {
        Todo::find().all(db).await
    }

    /// If ok, returns (todo models, num pages).
    pub async fn find_todos_in_page(
        db: &DbConn,
        page: u64,
        todos_per_page: u64,
    ) -> Result<(Vec<todo::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Todo::find()
            .order_by_asc(todo::Column::Id)
            .paginate(db, todos_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated todos
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
