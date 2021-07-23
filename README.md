# Taus

A [tauri](https://tauri.studio/en/) auto update server.

## Development

### Server
Rust nightly is required to build [rocket](https://rocket.rs/). The
[diesel](https://diesel.rs/) tool is required to be in your path to setup the
database. 

Setup a `.env` file according to the `.env.example` file. Run `diesel migration
run`. To run the server run `cargo run`.

### Frontend
The [yarn](https://yarnpkg.com/) package manager is required. Run
```
cd web
yarn
```
to install javascript dependencies.

Then `web/` is a [create-react-app](https://create-react-app.dev/) project so a
development server can be started with `yarn start`. Additionally `yarn format`
will format most of the files with prettier.

## Project management
See the [project board](https://github.com/gabcoh/taus/projects/1).
