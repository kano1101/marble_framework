pub async fn run<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>(
    fn_fut: FnFut,
    migrator: Option<Mitor>,
) -> anyhow::Result<()>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    use get_database_url_for_environment::get_database_url_for_environment;
    let url = get_database_url_for_environment().await?;

    use establish_aws_mysql_sqlx::get_connection_cache_or_establish;
    let pool = get_connection_cache_or_establish(&url).await?;

    if let Some(migrator) = migrator {
        migrator(pool).await?;
    }

    use lambda_http::{run, service_fn};
    run(service_fn(move |event| {
        let mut fn_fut = fn_fut.clone();
        async move {
            use parse_bearer_token::parse_bearer_token;
            let token = &parse_bearer_token(&event)?;

            let (parts, body) = event.into_parts();
            let origin = parts
                .headers
                .get("Origin")
                .ok_or(anyhow::anyhow!("missing Origin header"))?;
            let body = match body {
                lambda_http::Body::Text(string) => string,
                lambda_http::Body::Binary(v) => String::from_utf8(v.clone())?,
                lambda_http::Body::Empty => "".to_string(),
            };

            let result = fn_fut(pool, token.to_string(), body.to_string()).await?;

            let response = lambda_http::Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Methods", "OPTIONS,GET,POST")
                .header("Access-Control-Allow-Credentials", "true")
                .header("Access-Control-Allow-Origin", origin)
                .header(
                    "Access-Control-Allow-Headers",
                    "Origin,Authorization,Accept,X-Requested-With",
                )
                .body(lambda_http::Body::from(result.to_string()))?;

            Ok::<_, anyhow::Error>(response)
        }
    }))
    .await
    .map_err(|_| anyhow::anyhow!("failed to running lambda."))?;

    Ok(())
}
