mod discord;
mod quote;

use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tokio_cron_scheduler::{Job, JobScheduler};

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    let job = Job::new_async("0 */1 * * * * *", |_uuid, mut _l| {
        Box::pin(async move {
            println!("Running quote bot");
            let quote = quote::get_quote().await.unwrap();

            let quote_string = format!("Today's quote `{:?}`\nby `{:?}`", quote.q, quote.a);

            println!("Today's quote {:?} is by {:?}", quote.q, quote.a);

            discord::send_to_webhook(quote_string).await.unwrap();
        })
    })
    .unwrap();

    // Add async job
    sched.add(job).await?;

    sched.start().await?;

    // Start the scheduler
    // tokio::spawn(sched.start());

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3003));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Finished all cron task");

    Ok(())
}
