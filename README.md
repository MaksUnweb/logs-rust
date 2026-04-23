## Introduction
Welcome to my project on working with error logs of their Docker containers 🔥.
This project is a Backend in the form of a log handler with filtering to extract exactly the erroneous ones and upload them to a database and a web interface for beautiful and convenient output.

## Who will it be useful for? 

This project is distributed under the MIT license, meaning you can use it for your own purposes without buying any subscriptions or licenses, just clone the repository and follow the instructions to run it.

You can use:
- ✅ If you have a home server with Nginx or a database. 
- ✅ If you are a small organization and you need to solve the problem with log processing.
- ✅ If you are an enthusiast and want to find answers to questions about working with Docker in the context of Rust. 

## Instruction:

To run the project, you need to have the following components installed:

- docker
- docker-compose
- rust >=1.95.0
- cargo >= 1.95.0

1) You need to clone the project to your device using the standard git clone command. 
2) Next, you need to create environment variables, here is a list of necessary ones: 

```
POSTGRES_USER=your_login
POSTGRES_PASSWORD=your_password
POSTGRES_DB=your_database
CONTAINER_ID=Your_id_container
DB_URL=postgres://your_login:your_password@your_host/your_database?connect_timeout=your_time_connect
```
Based on the names of the variables, it is clear what exactly they mean.

3) Compile a project using `cargo build --release`.
4) Get the binary file of the project in `target/release/myapp`.

[!WARNING]  
Do not forget to create environment variables with valid data, and before launching the project, make sure that the database container, as well as the container from which the logs will be taken, are running and there are no conflicts between ports!

After launch (at the first launch), the administrator registration script will be launched (please register, otherwise the web interface and the backend for collecting logs will not start and you will not be able to access the web interface!).

## Screenshots:

Example of the Login Page:

[Login into WEB](./screenshots/login.png)


Example of output of erroneous logs:

[All problems](./screenshots/all_problems.png)

An example of the output of recent problems:

[Past problems](./screenshots/past_problems.png)

Example of a Hello block:

[Hello block](./screenshots/welcome.png)

Example of a block for testing the speed of a database:

[Test database](./screenshots/db_test.png)
