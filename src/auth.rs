use rspotify::{clients::BaseClient, ClientCredsSpotify, Credentials};

pub async fn auth_with_credentials(credentials: &str) -> Result<impl BaseClient, &'static str> {
    let (id, secret) = credentials
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
