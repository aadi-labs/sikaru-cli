#[path = "../../../cli/sikaru/commands.rs"]
mod commands;

use fern_cli_sdk::{app::CliApp, openapi::OpenApiBinding};

#[path = "../../cli/authoring.rs"]
mod authoring;

#[path = "../../cli/user_auth.rs"]
mod user_auth;

fn main() {
    let app = CliApp::new("sikaru-authoring")
        .binding(OpenApiBinding::new().commands(commands::description()));
    authoring::install(user_auth::install(app)).run()
}
