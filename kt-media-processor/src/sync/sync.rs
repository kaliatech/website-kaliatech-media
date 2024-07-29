use s3sync::config::args::parse_from_args;
use s3sync::config::Config;
use s3sync::pipeline::Pipeline;
use s3sync::types::token::create_pipeline_cancellation_token;

pub async fn do_s3_sync(local_dir: &str,
                        aws_profile: Option<&str>,
                        s3_url: &str) -> Result<(), Box<dyn std::error::Error>> {

    // You can use all the arguments for sync binary here.
    let mut args = vec!["s3sync", "--delete", local_dir, s3_url];

    if let Some(aws_profile) = aws_profile {
        args.push("--target-profile");
        args.push(aws_profile);
    }

    // sync library converts the arguments to Config.
    let config = Config::try_from(parse_from_args(args).unwrap()).unwrap();

    // Create a cancellation token for the pipeline.
    // You can use this token to cancel the pipeline.
    let cancellation_token = create_pipeline_cancellation_token();
    let mut pipeline = Pipeline::new(config.clone(), cancellation_token).await;

    // You can close statistics sender to stop statistics collection, if needed.
    // Statistics collection consumes some Memory, so it is recommended to close it if you don't need it.
    pipeline.close_stats_sender();

    // Run the pipeline. In this simple example, we run the pipeline synchronously.
    pipeline.run().await;

    if pipeline.has_error() {
        let errs = pipeline.get_errors_and_consume().unwrap();
        for err in errs {
            eprintln!("{}", err);
        }
        Err("Error with sync".into())
    } else {
        Ok(())
    }
}