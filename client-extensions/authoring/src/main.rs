#[path = "../../../cli/sikaru/commands.rs"]
mod commands;

use fern_cli_sdk::{app::CliApp, auth::BearerAuth, openapi::OpenApiBinding};

#[path = "../../cli/authoring.rs"]
mod authoring;

fn main() {
    let app = CliApp::new("sikaru-authoring")
        .auth(BearerAuth::new("BearerAuth").env("SIKARU_API_KEY"))
        .binding(OpenApiBinding::new().commands(commands::description()));
    authoring::install(app).run()
}
