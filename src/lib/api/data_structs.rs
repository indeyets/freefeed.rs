pub struct User {
    pub uuid: String,
    pub username: String,
    pub screen_name: String,
}

pub struct Attachment {
    pub id: String,
    pub file_name: String,
    pub file_size: u64,
    pub url: String,
}

pub struct Post {
    pub attachments: Vec<Attachment>,
    pub author: User,
    pub body: String,
}
