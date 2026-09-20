use fern_cli_sdk::{app::CliApp, auth::BearerAuth, openapi::OpenApiBinding};

#[path = "../../cli/authoring.rs"]
mod authoring;

fn main() {
    let app = CliApp::new("sikaru-authoring")
        .auth(BearerAuth::new("BearerAuth").env("SIKARU_API_KEY"))
        .binding(OpenApiBinding::new().spec(include_str!("../../../cli/sikaru/openapi0.json")));
    authoring::install(app).run()
}
