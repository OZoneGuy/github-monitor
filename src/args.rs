use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    #[arg(short, long, default_value = "./config.yaml")]
    pub config: String,
    #[arg(env = "APP_PAT", conflicts_with_all = [ "app-id", "app-secret" ])]
    pub pat: Option<String>,
    #[arg(env = "APP_ID", requires = "app-secret")]
    pub app_id: Option<String>,
    #[arg(env = "APP_SECRET", requires = "app-id")]
    pub app_secret: Option<String>,
}
