use dotenv::dotenv;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport};

/// Creates an email to ask for a verification code
pub fn verification_code_email(email: &str, name: &str, code: &str) -> Message {
	let username = get_email();
	Message::builder()
		.from(format!("Loop Team <{}>", username).parse().unwrap())
		.to(format!("{} <{}>", name, email).parse().unwrap())
		.subject("Loop Email Verification")
		.body(format!("Your Loop account verification code is {}", code))
		.unwrap()
}

/// Gets (loop's) email from the environment for sending emails
pub fn get_email() -> String {
	dotenv().ok();
	std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME is required to send emails")
}

/// Get the SMTP credentials from the environment
pub fn get_creds() -> Credentials {
	let username = get_email();
	let password =
		std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is required to send emails");
	Credentials::new(username, password)
}

/// Create a lettre mailer for sending emails. Gets the SMTP credentials from the environment.
pub fn make_mailer() -> SmtpTransport {
	dotenv().ok();
	let server = std::env::var("SMTP_SERVER").expect("SMTP_SERVER is required to send emails");
	let creds = get_creds();
	SmtpTransport::relay(&server)
		.unwrap()
		.credentials(creds)
		.build()
}
