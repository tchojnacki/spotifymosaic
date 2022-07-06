use rspotify::{clients::BaseClient, ClientCredsSpotify, Credentials};

pub async fn auth_with_client_creds(creds_str: &str) -> Result<impl BaseClient, &'static str> {
    let (id, secret) = creds_str
        .split_once(':')
        .ok_or("Invalid credentials format")?;

    let creds = Credentials::new(id, secret);
    let mut client = ClientCredsSpotify::new(creds);

    client
        .request_token()
        .await
        .or(Err("Authentication failed!"))?;

    Ok(client)
}
