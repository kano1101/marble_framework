struct Config<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    UserId: From<String> + Send + Sync,
    FnFut: FnMut(&'a sqlx::MySqlPool, UserId, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    _phantom: std::marker::PhantomData<&'a UserId>,
    fn_fut: FnFut,
    region: Option<&'static str>,
    user_pool_id: Option<String>,
    migrator: Option<Mitor>,
    origin: Option<String>,
    methods: Option<String>,
}
impl<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    Config<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    UserId: From<String> + Send + Sync,
    FnFut: FnMut(&'a sqlx::MySqlPool, UserId, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet + Clone,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    #[allow(dead_code)]
    fn fn_fut(&'a self) -> FnFut {
        self.fn_fut.clone()
    }
    fn region(&'a self) -> Option<&'static str> {
        self.region
    }
    fn user_pool_id(&'a self) -> Option<String> {
        self.user_pool_id.clone()
    }
    #[allow(dead_code)]
    fn migrator(&'a self) -> Option<Mitor> {
        self.migrator.as_ref().map(|m| m.clone())
    }
    #[allow(dead_code)]
    fn origin(&'a self) -> Option<String> {
        self.origin.clone()
    }
    #[allow(dead_code)]
    fn methods(&'a self) -> Option<String> {
        self.methods.clone()
    }
}

impl<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    From<ClientBuilder<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>>
    for Config<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    UserId: From<String> + Send + Sync,
    FnFut: FnMut(&'a sqlx::MySqlPool, UserId, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    fn from(
        from: ClientBuilder<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>,
    ) -> Self {
        Self {
            _phantom: from._phantom,
            fn_fut: from.fn_fut,
            region: from.region,
            user_pool_id: from.user_pool_id,
            migrator: from.migrator,
            origin: from.origin,
            methods: from.methods,
        }
    }
}

pub struct ClientBuilder<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    UserId: From<String> + Send + Sync,
    FnFut: FnMut(&'a sqlx::MySqlPool, UserId, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    _phantom: std::marker::PhantomData<&'a UserId>,
    fn_fut: FnFut,
    region: Option<&'static str>,
    user_pool_id: Option<String>,
    migrator: Option<Mitor>,
    origin: Option<String>,
    methods: Option<String>,
}
impl<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    ClientBuilder<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    UserId: From<String> + Send + Sync,
    FnFut: FnMut(&'a sqlx::MySqlPool, UserId, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    pub fn build(self) -> Client<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes> {
        let client = Client {
            config: self.into(),
        };
        client
    }
    pub fn new(fn_fut: FnFut) -> Self {
        Self {
            _phantom: std::marker::PhantomData::<&'a UserId>,
            fn_fut,
            region: None,
            user_pool_id: None,
            migrator: None,
            origin: None,
            methods: None,
        }
    }
    pub fn set_region(mut self, region: &'static str) -> Self {
        self.region = Some(region);
        self
    }
    pub fn set_user_pool_id(mut self, user_pool_id: &'a str) -> Self {
        self.user_pool_id = Some(user_pool_id.to_string());
        self
    }
    pub fn set_migrator(mut self, migrator: Mitor) -> Self {
        self.migrator = Some(migrator);
        self
    }
    pub fn set_origin(mut self, origin: &'a str) -> Self {
        self.origin = Some(origin.to_string());
        self
    }
    pub fn set_methods(mut self, methods: &'a str) -> Self {
        self.methods = Some(methods.to_string());
        self
    }
}

pub struct Client<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    UserId: From<String> + Send + Sync,
    FnFut: FnMut(&'a sqlx::MySqlPool, UserId, String) -> FnFutRet + Send + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    config: Config<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>,
}

impl<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
    Client<'a, UserId, FnFut, FnFutRet, FnFutRes, Mitor, MitorRet, MitorRes>
where
    UserId: From<String> + Send + Sync,
    FnFut: FnMut(&'a sqlx::MySqlPool, UserId, String) -> FnFutRet + Send + Sync + Clone,
    FnFutRet: std::future::Future<Output = anyhow::Result<FnFutRes>> + Send,
    FnFutRes: std::fmt::Display,
    Mitor: FnOnce(&'a sqlx::MySqlPool) -> MitorRet + Sync + Clone,
    MitorRet: std::future::Future<Output = anyhow::Result<MitorRes>> + Send,
{
    pub async fn run(&'a self) -> anyhow::Result<()> {
        use dotenv::dotenv;
        dotenv().ok();

        let region_string = std::env::var("REGION")?;
        let region_str = match &region_string[..] {
            "ap-northeast-1" => "ap-northeast-1",
            "ap-northeast-2" => "ap-northeast-2",
            "ap-northeast-3" => "ap-northeast-3",
            _ => unimplemented!(),
        };
        let region: &str = self.config.region().unwrap_or_else(|| region_str);

        let user_pool_id: String = match self.config.user_pool_id() {
            Some(id) => id,
            None => std::env::var("USER_POOL_ID")?,
        };
        let user_pool_id = user_pool_id.as_ref();

        use get_database_url_for_environment::get_database_url_for_environment;
        let url = get_database_url_for_environment(Some(region), &user_pool_id).await?;

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

                let methods = match self.config.methods.as_ref() {
                    Some(methods) => methods.clone(),
                    None => "GET".to_string(),
                };
                let headers = "Origin, Authorization, Accept, X-Requested-With, X-HTTP-Method-Override, Content-Type";

                let (parts, body) = event.into_parts();

                let origin = match self.config.origin.as_ref() {
                    Some(origin) => origin.clone(),
                    None => match std::env::var("ORIGIN") {
                        Ok(ok) => ok,
                        Err(_) => match parts
                        .headers
                        .get("Origin")
                        .ok_or(anyhow::anyhow!("missing Origin header"))?
                        .to_str().ok() {
                            Some(s) => s.to_string(),
                            None => "".to_string(),
                        },
                    }
                };
                let body = match body {
                    lambda_http::Body::Text(string) => string,
                    lambda_http::Body::Binary(v) => String::from_utf8(v.clone())?,
                    lambda_http::Body::Empty => "".to_string(),
                };

                let jwks_url =
                    format!("https://cognito-idp.{region}.amazonaws.com/{user_pool_id}/.well-known/jwks.json");

                use decode_token_by_jwks::decode_user_sub_from_token;
                let user_id = decode_user_sub_from_token(&token, &jwks_url).await?;

                let result = fn_fut(pool, user_id.into(), body.to_string()).await?;

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
