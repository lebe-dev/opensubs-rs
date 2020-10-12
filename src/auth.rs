pub mod auth {
    use crate::types::types::OperationResult;
    use crate::error::error::OperationError;

    const A_PROPERTY: &str = "a";
    const REDIRECT_PROPERTY: &str = "redirect";
    const USER_PROPERTY: &str = "user";
    const PASSWORD_PROPERTY: &str = "password";
    const REMEMBER_PROPERTY: &str = "remember";

    pub async fn login(client: &reqwest::Client, base_url: &str,
                       login: &str, password: &str) -> Result<(), Box<OperationError>> {
        info!("login to '{}'", base_url);

        let params = [
            (A_PROPERTY, "login"),
            (REDIRECT_PROPERTY, "/ru"),
            (USER_PROPERTY, login),
            (PASSWORD_PROPERTY, password),
            (REMEMBER_PROPERTY, "on"),
        ];

        let url = format!("{}/ru/login/redirect-ru", base_url);

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

                if status == reqwest::StatusCode::FOUND {
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
