struct Config<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    fn_fut: FnFut,
    migrator: Option<Mitor>,
    origin: Option<&'a str>,
    methods: Option<&'a str>,
}
impl<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    Config<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet + Clone,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    #[allow(dead_code)]
    fn fn_fut(&self) -> FnFut {
        self.fn_fut.clone()
    }
    #[allow(dead_code)]
    fn migrator(&self) -> Option<Mitor> {
        self.migrator.as_ref().map(|m| m.clone())
    }
    #[allow(dead_code)]
    fn origin(&self) -> Option<&'a str> {
        self.origin
    }
    #[allow(dead_code)]
    fn methods(&self) -> Option<&'a str> {
        self.methods
    }
}

impl<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    From<ClientBuilder<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>>
    for Config<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    fn from(from: ClientBuilder<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>) -> Self {
        Self {
            fn_fut: from.fn_fut,
            migrator: from.migrator,
            origin: from.origin,
            methods: from.methods,
        }
    }
}

pub struct ClientBuilder<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    fn_fut: FnFut,
    migrator: Option<Mitor>,
    origin: Option<&'a str>,
    methods: Option<&'a str>,
}
impl<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    ClientBuilder<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    pub fn build(self) -> Client<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes> {
        let client = Client {
            config: self.into(),
        };
        client
    }
    pub fn new(fn_fut: FnFut) -> Self {
        Self {
            fn_fut,
            migrator: None,
            origin: None,
            methods: None,
        }
    }
    pub fn migrator(mut self, migrator: Mitor) -> Self {
        self.migrator = Some(migrator);
        self
    }
    pub fn origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin);
        self
    }
    pub fn method(mut self, method: &'a str) -> Self {
        self.methods = Some(method);
        self
    }
}

pub struct Client<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    config: Config<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>,
}

impl<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    Client<'a, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    FnFut: FnMut(&'a sqlx::MySqlPool, String, String) -> FnFutRet + Send + Sync + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet + Sync + Clone,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    pub async fn run(&self) -> anyhow::Result<()> {
        use get_database_url_for_environment::get_database_url_for_environment;
        let url = get_database_url_for_environment().await?;

        use establish_aws_mysql_sqlx::get_connection_cache_or_establish;
        let pool = get_connection_cache_or_establish(&url).await?;

        if let Some(migrator) = self.config.migrator.clone() {
            migrator(pool).await?;
        }

        use lambda_http::{run, service_fn};
        run(service_fn(move |event| {
            let mut fn_fut = self.config.fn_fut.clone();
            async move {
                use parse_bearer_token::parse_bearer_token;
                let token = &parse_bearer_token(&event)?;

                let methods = match self.config.methods {
                    Some(methods) => methods,
                    None => "GET",
                };
                let headers = "Origin, Authorization, Accept, X-Requested-With, X-HTTP-Method-Override, Content-Type";

                let (parts, body) = event.into_parts();
                let origin = match self.config.origin {
                    Some(origin) => origin,
                    None => match parts
                        .headers
                        .get("Origin")
                        .ok_or(anyhow::anyhow!("missing Origin header"))?
                        .to_str().ok() {
                            Some(s) => s,
                            None => "",
                        }
                };
                let body = match body {
                    lambda_http::Body::Text(string) => string,
                    lambda_http::Body::Binary(v) => String::from_utf8(v.clone())?,
                    lambda_http::Body::Empty => "".to_string(),
                };

                let result = fn_fut(pool, token.to_string(), body.to_string()).await?;

                let mut builder = {
                    lambda_http::Response::builder()
                        .status(200)
                        .header("Content-Type", "application/json")
                        .header("Access-Control-Allow-Headers", headers)
                        .header("Access-Control-Allow-Methods", methods)
                };
                if !origin.is_empty() {
                    builder = builder.header("Access-Control-Allow-Origin", origin);
                }
                let response = builder
                    .body(lambda_http::Body::from(result.to_string()))?;

                Ok::<_, anyhow::Error>(response)
            }
        }))
            .await
            .map_err(|_| anyhow::anyhow!("failed to running lambda."))?;

        Ok(())
    }
}
