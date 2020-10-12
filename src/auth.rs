pub mod auth {
    use crate::error::error::OperationError;

    const A_PROPERTY: &str = "a";
    const REDIRECT_PROPERTY: &str = "redirect";
    const USER_PROPERTY: &str = "user";
    const PASSWORD_PROPERTY: &str = "password";
    const REMEMBER_PROPERTY: &str = "remember";

    /// Login to opensubtitles.org
    ///
    /// # Examples
    /// ```
    /// use opensubs_rs::BASE_URL;
    /// login_to_opensubs(&client, BASE_URL, "username", "supppaPazzWourd");
    /// ```
    pub async fn login_to_opensubs(client: &reqwest::Client, base_url: &str,
                                   login: &str, password: &str) -> Result<(), Box<OperationError>> {
        info!("login to '{}'", base_url);

        let params = [
            (A_PROPERTY, "login"),
            (REDIRECT_PROPERTY, "/ru"),
            (USER_PROPERTY, login),
            (PASSWORD_PROPERTY, password),
            (REMEMBER_PROPERTY, "on"),
        ];

        let url = format!("{}/ru/login/redirect-%7Cru", base_url);

        match client.post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send().await {
            Ok(resp) => {
                let rsp_header = resp.headers();

                debug!("response Header: {:?}", rsp_header);

                let cookies = resp.cookies();

                for cookie in cookies {
                    debug!("cookie: '{}' value '{}'", cookie.name(), cookie.value());
                }

                let status: reqwest::StatusCode = resp.status();

                debug!("status code '{}'", status);

                if status == reqwest::StatusCode::OK {
                    let html = resp.text().await.unwrap();

                    trace!("---[AUTH RESPONSE]---");
                    trace!("{}", &html);
                    trace!("---[/AUTH RESPONSE]---");

                    Ok(())

                } else {
                    error!("error, response code was {}", status);
                    Err(Box::from(OperationError::Authentication))
                }
            }
            Err(e) => {
                error!("authentication error, unable to connect: '{}'", e);
                Err(Box::from(OperationError::Error))
            }
        }
    }
}
