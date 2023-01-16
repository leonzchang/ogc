mod bot;

use crate::{cli::SharedParams, mono::bot::CheaterBot};

use std::{io::Write, path::PathBuf, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware,
    rt::System,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use ansi_term::Colour;
use chrono::Utc;
use clap::{Parser, ValueHint};
use fantoccini::{ClientBuilder, Locator};
use log::{logger, Level};
use tokio::{
    sync::{Mutex, RwLock},
    time::{sleep, Duration},
};

#[derive(Debug, Parser)]
pub struct Opts {
    /// Host string in "${HOST}:${PORT}" format.
    #[clap(long, default_value = "127.0.0.1:3000", env = "OGC_HOST")]
    host: String,

    #[clap(long, env = "OGC_JWT_PRIV_FILE", value_parser, value_hint = ValueHint::FilePath)]
    jwt_priv_file: Option<PathBuf>,

    #[clap(long, env = "OGC_JWT_PUB_FILE", value_parser, value_hint = ValueHint::FilePath)]
    jwt_pub_file: Option<PathBuf>,
}

pub fn run(shared: SharedParams, opts: Opts) -> anyhow::Result<()> {
    init_logger("warn,oracle-core=info,oracle=info", true);

    let system = System::new();
    // let pg_pool = system.block_on(connect_and_migrate(&shared.database_url, 5))?;

    system.block_on(async {
        let bot = CheaterBot::new(shared.config_path, shared.webdriver_url.as_deref()).await?;
        log::error!("cheater {:?}", bot);

        bot.login().await?;

        let resource = bot.get_resource().await?;
        log::info!("resources {:?}", resource);
        let infrastructure = bot.get_infrastructure_level().await?;
        log::info!("infrastructure {:?}", infrastructure);
        let facility = bot.get_facility_level().await?;
        log::info!("facility {:?}", facility);
        let technology = bot.get_technology_level().await?;
        log::info!("technology {:?}", technology);

        Ok(())
        // build_http_service(&opts.host, pg_pool, generation_engine, grpc_client)
        //     .await
        //     .map_err(Into::into)
    })
}

// async fn build_http_service(
//     host: &str,
//     pg_pool: PgPool,
//     generation_engine: Arc<RwLock<GenerationEgine>>,
//     grpc_client: GrpcClient,
// ) -> std::io::Result<()> {
//     HttpServer::new(move || {
//         App::new()
//             .app_data(Data::new(pg_pool.clone()))
//             .app_data(Data::new(generation_engine.clone()))
//             .app_data(Data::new(Mutex::new(grpc_client.clone())))
//             .wrap(middleware::Logger::default())
//             .route("/healthz", web::get().to(HttpResponse::Ok))
//             .service(
//                 web::scope("secure")
//                     .wrap(
//                         Cors::default()
//                             .allowed_origin("http://localhost:1234")
//                             .allowed_origin("http://127.0.0.1:1234")
//                             .allowed_methods(vec!["POST", "GET", "PUT", "DELETE"])
//                             .allowed_headers(vec![
//                                 header::CONTENT_TYPE,
//                                 header::AUTHORIZATION,
//                                 header::ACCEPT,
//                             ])
//                             .max_age(3600)
//                             .supports_credentials(),
//                     )
//                     .configure(crate::api::routes("api")),
//             )
//     })
//     .bind(host)?
//     .run()
//     .await
// }

pub fn init_logger(pattern: &str, deep: bool) {
    let mut builder = env_logger::Builder::new();
    builder.parse_filters(pattern);
    if let Ok(lvl) = std::env::var("RUST_LOG") {
        builder.parse_filters(&lvl);
    }

    if deep {
        builder.format(move |buf, record| {
            let time_now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let level = match record.level() {
                Level::Error => Colour::Red.bold().paint("ERR"),
                Level::Warn => Colour::Yellow.bold().paint("WRN"),
                Level::Info => Colour::Green.bold().paint("INF"),
                Level::Debug => Colour::Cyan.bold().paint("DBG"),
                Level::Trace => Colour::White.bold().paint("TRC"),
            };
            let output = format!(
                "[{}] {} {}\n  {}|{}:{}",
                level,
                Colour::Blue.bold().paint(time_now),
                record.args(),
                record.module_path().unwrap_or("UNKNOWN_MODULE"),
                record.file().unwrap_or("UNKNOWN_FILE"),
                record.line().unwrap_or(0),
            );
            writeln!(buf, "{}", output)
        });
    } else {
        builder.format(move |buf, record| {
            let time_now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let level = match record.level() {
                Level::Error => Colour::Red.bold().paint("ERR"),
                Level::Warn => Colour::Yellow.bold().paint("WRN"),
                Level::Info => Colour::Green.bold().paint("INF"),
                Level::Debug => Colour::Cyan.bold().paint("DBG"),
                Level::Trace => Colour::White.bold().paint("TRC"),
            };
            let output = format!(
                "[{}] {} {}",
                level,
                Colour::Blue.bold().paint(time_now),
                record.args(),
            );
            writeln!(buf, "{}", output)
        });
    };

    if builder.try_init().is_err() {
        log::info!("Global logger already initialized.  Skipping");
    }
}
