use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub task: Vec<String>,
    pub stat: i32,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct NewTask {
    pub id: i32,
    pub task: Vec<String>,
    pub stat: i32,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct UpdateTask {
    pub task: Vec<String>,
    pub stat: i32,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct UpdatePartialTask {
    #[sqlx(default)]
    pub task: Option<Vec<String>>,
    #[sqlx(default)]
    pub stat: Option<i32>,
}