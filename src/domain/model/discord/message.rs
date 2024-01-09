#[derive(Debug, Clone, Copy)]
pub struct DiscordMessage {
	pub message_id: u64,
	pub thread_id: Option<u64>
}
