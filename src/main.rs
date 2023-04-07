use rustic_logger::{log, TSeverity};
use tq_code_generator_core::generate_random_code;
use warp::{Filter, Rejection, Reply};
use serde::{Serialize};

#[derive(Serialize)]
struct CodeResponse {
    code: String,
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();

    rt.block_on(async {
        let generate_code = warp::path!("generate" / usize)
            .map(|length: usize| {
                let code = generate_random_code(length);
                let code_log = format!("successfully generate random code: {:?}", code);
                let code_log_box = code_log.into_boxed_str();
                let code_log_static: &'static str = Box::leak(code_log_box);
                log(
                    "app-log.log",
                    code_log_static,
                    Some(TSeverity::INFO),
                    None,
                ).unwrap();
                warp::reply::json(&CodeResponse { code })
            })
            .recover(handle_rejection);

        warp::serve(generate_code)
            .run(([127, 0, 0, 1], 8000))
            .await;
    });
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    let error_message = format!("internal server error: {:?}", err);
    let error_message_box = error_message.into_boxed_str();
    let error_message_static: &'static str = Box::leak(error_message_box);
    let log_result = log(
        "app-log.log",
        error_message_static,
        Some(TSeverity::ERROR),
        None,
    );
    if let Err(e) = log_result {
        eprintln!("Failed to write app-log.log file: {}", e);
    }
    Ok(warp::reply::with_status(
        warp::reply::json(&"internal server error"),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
