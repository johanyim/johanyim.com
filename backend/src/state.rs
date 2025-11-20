use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, AsyncSmtpTransport, Message,
    SmtpTransport, Tokio1Executor,
};

#[derive(Clone)]
pub struct AppState {
    //pub db: deadpool_tiberius::Pool,
    pub mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl AppState {
    pub fn new() -> Self {
        let mailer = {
            let username = std::env::var("EMAIL_USER").expect("EMAIL_USER is not set");
            let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD is not set");
            let creds = Credentials::new(username, password);
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&"smtp.gmail.com")
                .unwrap()
                .port(587)
                .credentials(creds)
                .build()
        };

        Self { mailer }
    }
}
