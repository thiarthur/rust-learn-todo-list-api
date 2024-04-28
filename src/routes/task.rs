use ntex::web::{self, Error, HttpResponse};

use crate::controller::task::{create_task, delete_task, find_task_by_id, list_tasks, update_task};
use crate::db::DbPool;
use crate::error::{create_internal_error, NotFoundError};
use crate::model::task::NewTask;
use crate::response::SuccessResponse;
use web::types::{Json, Path, State};

#[web::get("/tasks/{task_id}")]
async fn get_task_route(pool: State<DbPool>, task_id: Path<i32>) -> Result<HttpResponse, Error> {
    let task_id = task_id.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let task = web::block(move || find_task_by_id(task_id, &mut conn)).await?;

    if let Some(task) = task {
        return Ok(HttpResponse::Ok().json(&task));
    }

    Err(create_internal_error(NotFoundError {
        message: format!("No task found with id: {}", task_id),
    }))
}

#[web::get("/tasks")]
async fn list_tasks_route(pool: State<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let tasks = web::block(move || list_tasks(&mut conn)).await?;

    return Ok(HttpResponse::Ok().json(&tasks));
}

#[web::post("/tasks")]
async fn create_task_route(
    pool: State<DbPool>,
    data: Json<NewTask>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let tasks = web::block(move || create_task(&data, &mut conn)).await?;

    return Ok(HttpResponse::Ok().json(&tasks));
}

#[web::put("/tasks/{task_id}")]
async fn update_task_route(
    pool: State<DbPool>,
    task_id: Path<i32>,
    data: Json<NewTask>,
) -> Result<HttpResponse, Error> {
    let task_id = task_id.into_inner();

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let task = web::block(move || update_task(task_id, &data, &mut conn)).await?;

    if let Some(task) = task {
        return Ok(HttpResponse::Ok().json(&task));
    }

    Err(create_internal_error(NotFoundError {
        message: format!("No task found with id: {}", task_id),
    }))
}

#[web::delete("/tasks/{task_id}")]
async fn delete_task_route(pool: State<DbPool>, task_id: Path<i32>) -> Result<HttpResponse, Error> {
    let task_id = task_id.into_inner();

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let affected_rows = web::block(move || delete_task(task_id, &mut conn)).await?;

    if affected_rows > 0 {
        return Ok(HttpResponse::Ok().json(&SuccessResponse { success: true }));
    }

    Err(create_internal_error(NotFoundError {
        message: format!("No task found with id: {}", task_id),
    }))
}
