use chrono::{DateTime, Duration, Utc};
use ed25519_dalek::{
	ed25519::signature::rand_core::OsRng, Signature, Signer, SigningKey, Verifier, VerifyingKey
};
use sea_orm::ActiveValue;

use crate::{
	adapter::mysql::model::{gd_account_link, user},
	domain::model::error::discord::discord_error::DiscordError
};

#[derive(Debug, Clone)]
pub struct DiscordUser {
	pub discord_user_id: u64,
	pub gd_player_id: Option<u64>,
	pub last_request_time: Option<DateTime<Utc>>,
	pub is_gd_account_linked: bool
}

impl Into<user::ActiveModel> for DiscordUser {
	fn into(self) -> user::ActiveModel {
		user::ActiveModel {
			discord_id: ActiveValue::Set(self.discord_user_id),
			timestamp: ActiveValue::Set(if let Some(last_request_time) = self.last_request_time {
				Some(last_request_time)
			} else {
				None
			}),
			gd_player_id: ActiveValue::Set(self.gd_player_id),
			is_gd_account_linked: ActiveValue::Set(i8::from(self.is_gd_account_linked))
		}
	}
}

impl From<user::Model> for DiscordUser {
	fn from(value: user::Model) -> Self {
		Self {
			discord_user_id: value.discord_id,
			gd_player_id: value.gd_player_id,
			last_request_time: value.timestamp,
			is_gd_account_linked: if value.is_gd_account_linked != 0 {
				true
			} else {
				false
			}
		}
	}
}

impl DiscordUser {
	pub fn new(discord_user_id: u64) -> Self {
		Self {
			discord_user_id,
			gd_player_id: None,
			last_request_time: None,
			is_gd_account_linked: false
		}
	}
}

#[derive(Debug, Clone)]
pub struct GDAccountLink {
	pub discord_user_id: u64,
	pub gd_player_id: u64,
	pub gd_account_challenge: String,
	gd_account_hash: String,
	gd_account_public_key: [u8; 32],
	pub is_gd_account_linked: bool,
	pub expiry: DateTime<Utc>
}

impl Into<gd_account_link::ActiveModel> for GDAccountLink {
	fn into(self) -> gd_account_link::ActiveModel {
		gd_account_link::ActiveModel {
			discord_id: ActiveValue::Set(self.discord_user_id),
			gd_player_id: ActiveValue::Set(self.gd_player_id),
			is_gd_account_linked: ActiveValue::Set({
				if self.is_gd_account_linked {
					1
				} else {
					0
				}
			}),
			gd_account_challenge: ActiveValue::Set(self.gd_account_challenge),
			gd_account_hash: ActiveValue::Set(self.gd_account_hash),
			gd_account_public_key: ActiveValue::Set(Vec::from(self.gd_account_public_key)),
			expiry: ActiveValue::Set(self.expiry)
		}
	}
}

impl From<gd_account_link::Model> for GDAccountLink {
	fn from(value: gd_account_link::Model) -> Self {
		Self {
			discord_user_id: value.discord_id,
			gd_player_id: value.gd_player_id,
			gd_account_challenge: value.gd_account_challenge,
			gd_account_hash: value.gd_account_hash,
			gd_account_public_key: <[u8; 32]>::try_from(value.gd_account_public_key).unwrap(),
			is_gd_account_linked: if value.is_gd_account_linked != 0 {
				true
			} else {
				false
			},
			expiry: value.expiry
		}
	}
}

impl GDAccountLink {
	pub fn new(discord_user_id: u64, gd_player_id: u64) -> Self {
		Self {
			discord_user_id,
			gd_player_id,
			gd_account_hash: "".to_string(),
			gd_account_challenge: "".to_string(),
			gd_account_public_key: [0u8; 32],
			is_gd_account_linked: false,
			expiry: Utc::now() + Duration::minutes(15)
		}
	}

	pub fn generate_new_account_link(&mut self) {
		// Generate keypair
		let mut csprng = OsRng;
		let signing_key = SigningKey::generate(&mut csprng);

		// Generate public key phrase
		let public_key_bytes = signing_key.verifying_key().to_bytes();
		let public_key_bs58_string = bs58::encode(&public_key_bytes).into_string();

		// Sign public key phrase using private signing key
		let signature = signing_key.sign(public_key_bs58_string.as_bytes());
		let gd_account_hash = bs58::encode(signature.to_bytes()).into_string();

		self.gd_account_hash = gd_account_hash;
		self.gd_account_challenge = public_key_bs58_string;
		self.gd_account_public_key = public_key_bytes;
	}

	pub fn verify_account_link(
		&self,
		gd_account_challenge: &str
	) -> Result<bool, Box<dyn std::error::Error>> {
		if self.expiry.lt(&Utc::now()) {
			warn!("GD account link has expired");
			return Err(Box::new(DiscordError::GDAccountLinkExpired));
		}

		let signature_bytes_vec = bs58::decode(&self.gd_account_hash).into_vec()?;
		let signature_bytes: [u8; 64] = signature_bytes_vec.as_slice().try_into()?;
		let signature = Signature::from_bytes(&signature_bytes);

		let signing_key = VerifyingKey::from_bytes(&self.gd_account_public_key)?;

		if signing_key
			.verify(gd_account_challenge.as_bytes(), &signature)
			.is_ok()
		{
			Ok(true)
		} else {
			Ok(false)
		}
	}
}

pub struct DiscordGDAccountLink {
	pub discord_user_id: u64,
	pub gd_player_id: u64,
	pub gd_username: String,
	pub gd_account_challenge: String
}

impl DiscordGDAccountLink {
	pub fn new(
		discord_user_id: u64,
		gd_player_id: u64,
		gd_username: String,
		gd_account_challenge: String
	) -> Self {
		Self {
			discord_user_id,
			gd_player_id,
			gd_username,
			gd_account_challenge
		}
	}
}

#[cfg(test)]
mod gd_account_link_tests {
	use super::*;

	const TEST_DISCORD_USER_ID: u64 = 164072941645070336;
	const TEST_GD_PLAYER_ID: u64 = 3713125;

	#[test]
	fn generate_new_account_link_populates_account_link_data() {
		let mut test_gd_account_link = GDAccountLink::new(TEST_DISCORD_USER_ID, TEST_GD_PLAYER_ID);

		assert!(test_gd_account_link.gd_account_hash.is_empty());
		assert!(test_gd_account_link.gd_account_challenge.is_empty());
		assert_eq!(test_gd_account_link.gd_account_public_key, [0u8; 32]);

		test_gd_account_link.generate_new_account_link();

		assert!(!test_gd_account_link.gd_account_hash.is_empty());
		assert!(!test_gd_account_link.gd_account_challenge.is_empty());
		assert_ne!(test_gd_account_link.gd_account_public_key, [0u8; 32]);
	}

	#[test]
	fn verify_new_account_link_verifies_successfully() {
		let mut test_gd_account_link = GDAccountLink::new(TEST_DISCORD_USER_ID, TEST_GD_PLAYER_ID);
		test_gd_account_link.generate_new_account_link();
		let test_account_challenge = test_gd_account_link.gd_account_challenge.clone();

		let result = test_gd_account_link.verify_account_link(&test_account_challenge);

		assert!(result.is_ok());
		assert!(result.unwrap());
	}

	#[test]
	fn verify_new_account_link_does_not_verify_successfully_if_expiry_has_passed() {
		let mut test_gd_account_link = GDAccountLink::new(TEST_DISCORD_USER_ID, TEST_GD_PLAYER_ID);
		test_gd_account_link.generate_new_account_link();
		test_gd_account_link.expiry = Utc::now() - Duration::minutes(30);
		let test_account_challenge = test_gd_account_link.gd_account_challenge.clone();

		match test_gd_account_link.verify_account_link(&test_account_challenge) {
			Ok(_) => assert!(false),
			Err(error) => assert!(error.downcast_ref::<DiscordError>().is_some())
		}
	}

	#[test]
	fn verify_new_account_link_does_not_verify_successfully_if_challenge_is_wrong() {
		let mut test_gd_account_link = GDAccountLink::new(TEST_DISCORD_USER_ID, TEST_GD_PLAYER_ID);
		test_gd_account_link.generate_new_account_link();

		let result = test_gd_account_link.verify_account_link("SOME_WRONG_CHALLENGE");

		assert!(result.is_ok());
		assert!(!result.unwrap());
	}
}
