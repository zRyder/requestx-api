use rocket_framework::{Build, Rocket};

use crate::adapter::controller::internal::{
	internal_level_request_controller, internal_level_review_controller,
	internal_moderator_controller, internal_request_manager_controller
};

pub fn mount_internal_controllers(rocket: Rocket<Build>) -> Rocket<Build> {
	rocket.mount(
		"/api/v1/internal",
		routes![
			internal_level_request_controller::update_level_request_message_id,
			internal_level_review_controller::update_level_review_message_id,
			internal_moderator_controller::send_level,
			internal_request_manager_controller::update_request_cooldown
		]
	)
}
