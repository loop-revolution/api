use dotenv::dotenv;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport};

/// Creates an email to ask for a verification code
pub fn verification_code_email(email: &str, name: &str, code: &str) -> Message {
	Message::builder()
		.from("Loop Team <team@loop.page>".parse().unwrap())
		.to(format!("{} <{}>", name, email).parse().unwrap())
		.subject("Loop Email Verification")
		.body(format!("Your Loop account verification code is {}", code))
		.unwrap()
}

pub fn get_creds() -> Credentials {
	dotenv().ok();
	let username =
		std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME is required to send emails");
	let password =
		std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is required to send emails");
	Credentials::new(username, password)
}

pub fn make_mailer() -> SmtpTransport {
	dotenv().ok();
	let server = std::env::var("SMTP_SERVER").expect("SMTP_SERVER is required to send emails");
	let creds = get_creds();
	SmtpTransport::relay(&server)
		.unwrap()
		.credentials(creds)
		.build()
}
