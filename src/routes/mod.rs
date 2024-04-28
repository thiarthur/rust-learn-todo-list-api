use ntex::web;

pub mod task;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(task::get_task_route);
    cfg.service(task::list_tasks_route);
    cfg.service(task::create_task_route);
    cfg.service(task::update_task_route);
    cfg.service(task::delete_task_route);
}
