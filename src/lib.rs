pub async fn run<'a, FnFut, Fut>(fn_fut: FnFut) -> anyhow::Result<()>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> Fut + Send + Clone,
    Fut: std::future::Future<Output = anyhow::Result<String>> + Send,
{
    use get_database_url_for_environment::get_database_url_for_environment;
    let url = get_database_url_for_environment().await?;

    use establish_aws_mysql_sqlx::get_connection_cache_or_establish;
    let pool = get_connection_cache_or_establish(&url).await?;

    use lambda_http::{run, service_fn};
    run(service_fn(move |event| {
        let mut fn_fut = fn_fut.clone();
        async move {
            use parse_bearer_token::parse_bearer_token;
            let token = &parse_bearer_token(&event)?;

            let body = match event.body() {
                lambda_http::Body::Text(string) => string,
                _ => unimplemented!(),
            };

            let result = fn_fut(pool, token.to_string(), body.to_string()).await?;

            let response = lambda_http::Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(lambda_http::Body::from(result))?;

            Ok::<_, anyhow::Error>(response)
        }
    }))
    .await
    .map_err(|_| anyhow::anyhow!("failed to running lambda."))?;

    Ok(())
}
