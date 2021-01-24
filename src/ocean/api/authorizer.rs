use crate::types::UserCode;

pub fn authorize(method: &String, user_code: &UserCode) -> bool {
    let allowed_code = match method.as_ref() {
        "mandela.update" => UserCode::User,
        "mandela.delete" => UserCode::Admin,
        "mandela.mark" => UserCode::User,
        "mandela.vote" => UserCode::User,
        "user.getOne" => UserCode::User,
        "user.update" => UserCode::User,
        "user.updateToken" => UserCode::User,
        "comment.update" => UserCode::User,
        "comment.delete" => UserCode::User,
        "forum.category.create" => UserCode::Admin,
        "forum.category.update" => UserCode::Admin,
        "forum.category.delete" => UserCode::Admin,
        "forum.section.create" => UserCode::Admin,
        "forum.section.update" => UserCode::Admin,
        "forums.section.delete" => UserCode::Admin,
        "forum.topic.create" => UserCode::Anonym,
        "forum.topic.update" => UserCode::User,
        "forums.topic.delete" => UserCode::User,
        "forums.topic.vote" => UserCode::User,
        "forum.post.create" => UserCode::Anonym,
        "forum.post.update" => UserCode::User,
        "forums.post.delete" => UserCode::User,
        _ => UserCode::Anonym,
    };

    user_security_order(user_code) >= user_security_order(&allowed_code)
}

fn user_security_order(user_code: &UserCode) -> u8 {
    match user_code {
        UserCode::Admin => 3,
        UserCode::User => 2,
        UserCode::Anonym => 1,
    }
}
