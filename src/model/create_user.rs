#[derive(FromForm)]
///Struct that is utilized to create new users. Post requests made to the /create/user endpoint. Data here will come from the create account form
pub struct CreateUser {
    ///User name of the prospective user, this should be unique
    pub user_name: String,

    ///Email of the prospective user, this should be unique
    pub email: String,

    ///Password of the prospective user, this is encoded and should not be used anywhere in code. This will get hashed as soon as the create user process is invoked.
    pub password: String,
}