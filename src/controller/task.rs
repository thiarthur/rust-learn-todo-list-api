use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::model::task::NewTask;
use crate::model::task::Task;

pub fn find_task_by_id(
    task_id: i32,
    conn: &mut PgConnection,
) -> Result<Option<Task>, diesel::result::Error> {
    use crate::schema::tasks::dsl::*;
    return Ok(tasks
        .filter(id.eq(task_id))
        .first::<Task>(conn)
        .optional()?);
}

pub fn list_tasks(conn: &mut PgConnection) -> Result<Vec<Task>, diesel::result::Error> {
    use crate::schema::tasks::dsl::*;

    return Ok(tasks.load::<Task>(conn)?);
}

pub fn create_task(
    new_task: &NewTask,
    conn: &mut PgConnection,
) -> Result<Task, diesel::result::Error> {
    use crate::schema::tasks::dsl::*;

    return Ok(diesel::insert_into(tasks)
        .values(new_task)
        .get_result(conn)?);
}

pub fn update_task(
    task_id: i32,
    new_task: &NewTask,
    conn: &mut PgConnection,
) -> Result<Option<Task>, diesel::result::Error> {
    use crate::schema::tasks::dsl::*;

    return Ok(diesel::update(tasks.filter(id.eq(task_id)))
        .set((
            title.eq(&new_task.title),
            description.eq(&new_task.description),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .get_result(conn)
        .optional()?);
}
