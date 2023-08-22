mod discord;
mod quote;

use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    let job = Job::new_async("0 0 * * * * *", |_uuid, mut _l| {
        Box::pin(async move {
            println!("Running quote bot");
            let quote = quote::get_quote().await.unwrap();

            let quote_string = format!("Today's quote `{:?}`\nby `{:?}`", quote.q, quote.a);

            println!("Today's quote {:?} is by {:?}", quote.q, quote.a);

            discord::send_to_webhook(quote_string).await.unwrap();
        })
    })
    .unwrap();

    let job2 = Job::new_async("* * * * * * *", |uuid, mut l| {
        Box::pin(async move {
            println!("I run async every 7 seconds");

            // Query the next execution time for this job
            let next_tick = l.next_tick_for_job(uuid).await;
            match next_tick {
                Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                _ => println!("Could not get next tick for 7s job"),
            }
        })
    })
    .unwrap();

    // Add async job
    sched.add(job).await?;
    // Add async job
    sched
        .add(Job::new_async("1/7 * * * * *", |uuid, mut l| {
            Box::pin(async move {
                println!("I run async every 7 seconds");

                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                    _ => println!("Could not get next tick for 7s job"),
                }
            })
        })?)
        .await?;

    sched.start().await?;

    // Start the scheduler
    // tokio::spawn(sched.start());

    println!("Finished all cron task");

    Ok(())
}
