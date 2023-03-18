# I Read

## A sample fullstack app with an unusual stack

This repository contains the code for a typical fullstack application with a twist, the stack consists on:

- **Backend**: [Rust][rust], [Actix][actix] and [Async-graphql][async-graphql].
- **Frontend**: [Flutter][flutter]/[Dart][dart].
- **Database**: _Probably_ [PostgreSQL][postgresql].
- It will _probably_ be deployed using Docker containers in AWS, Heroku or similar.

Besides the directory for backend and frontend, the repository also contains a documentation directory, this latter directory has details in how to run the application locally, and step-by-step details in how it was built, design decisions that were made and why.

To keep the application simple while touching several interesting concepts about a fullstack application, the application will help users keep a record of books that they have read.

[actix]: https://actix.rs
[async-graphql]: https://async-graphql.github.io/async-graphql/en/index.html
[dart]: https://dart.dev
[flutter]: https://flutter.dev
[postgresql]: https://www.postgresql.org
[rust]: https://www.rust-lang.org
