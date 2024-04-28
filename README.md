# Task Manager

This is a command-line task manager written in Rust. It allows users to create, list, edit, and remove tasks, with a simple menu-driven interface.

**This project was created for rust learning purposes**

## Features

-   Create tasks with a title and description
-   List all tasks with their details
-   Edit existing tasks
-   Clear console and display menu options
-   Remove tasks by id

## Getting Started

To get started with this project, ensure you have Rust and Cargo installed. Follow the steps below to clone the repository and run the project.

### Prerequisites

-   Rust and Cargo installed. [Install Rust](https://www.rust-lang.org/tools/install)

### Installation

1.  Clone the repository:

    ```bash
    git clone git@github.com:thiarthur/rust-learn-todo-list-api.git
    ```

2.  Change to the project directory:

    ```bash
    cd rust-learn-todo-list-api
    ```

3.  Set up the `.env` file for environment variables:

    -   Create a `.env` file in the project root.
    -   Add the `DATABASE_URL` for connecting to your PostgreSQL database.
    -   **Optional:**
        -   Add the `PORT` to change de running API port (default: 8000).
        -   Add the `HOST` to change de running API port (default: "127.0.0.1").

4.  Set up Diesel with the correct database schema:

    ```bash
    cargo install diesel_cli --no-default-features --features postgres
    diesel migration run
    ```

5.  Build the project with Cargo:

    ```bash
    cargo build
    ```

6.  Run the project:

    ```bash
    cargo run
    ```

## Usage

After running the project, you can interact with the REST API to create, list, edit, or remove tasks. Use tools like `curl` or Postman to send HTTP requests to the API endpoints.

-   `GET /tasks/{task_id}` to retrieve a specific task.
-   `GET /tasks` to list all tasks.
-   `POST /tasks` to create a new task (send JSON with `title` and `description`).
-   `PUT /tasks/{task_id}` to update an existing task.
-   `DELETE /tasks/{task_id}` to delete an existing task.

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please follow these steps:

1. Fork the repository on GitHub.
2. Create a new branch for your changes.
3. Commit your changes to the new branch.
4. Submit a pull request with a description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For questions or support, feel free to open an issue on GitHub or reach out via email: thiarthur1@gmail.com.
