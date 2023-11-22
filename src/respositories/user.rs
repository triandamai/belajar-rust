pub struct UserRepository {
    pub name: String,
    pub email: String,
}

impl UserRepository {
    pub fn create() -> UserRepository {
        UserRepository {
            name: String::from("Trian Dmaai"),
            email: String::from("trian@gmail.com"),
        }
    }

    pub async fn get_user(&self) -> std::option::Option<String> {
        Some(self.name.clone())
    }
}
