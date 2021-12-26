use crate::config;
use crate::types::UserCode;

pub fn authorize(method: &str, user_code: &UserCode) -> bool {
    let anonym_allowed = config::CONFIG.server.anonym_allowed;

    let allowed_code = match method {
        "mandela.create" => {
            if anonym_allowed {
                UserCode::Anonym
            } else {
                UserCode::User
            }
        }
        "mandela.update" => UserCode::User,
        "mandela.delete" => UserCode::Admin,
        "mandela.mark" => UserCode::User,
        "mandela.vote" => UserCode::User,
        "mandela.updateTrash" => UserCode::Admin,
        "mandela.getVoteUsers" => UserCode::Admin,
        "user.logout" => UserCode::User,
        "user.update" => UserCode::Admin,
        "user.updateToken" => UserCode::User,
        "user.updateProfile" => UserCode::User,
        "comment.create" => {
            if anonym_allowed {
                UserCode::Anonym
            } else {
                UserCode::User
            }
        }
        "comment.update" => UserCode::User,
        "comment.delete" => UserCode::User,
        "forum.category.create" => UserCode::Admin,
        "forum.category.update" => UserCode::Admin,
        "forum.category.delete" => UserCode::Admin,
        "forum.section.create" => UserCode::Admin,
        "forum.section.update" => UserCode::Admin,
        "forums.section.delete" => UserCode::Admin,
        "forum.topic.create" => {
            if anonym_allowed {
                UserCode::Anonym
            } else {
                UserCode::User
            }
        }
        "forum.topic.update" => UserCode::User,
        "forums.topic.delete" => UserCode::User,
        "forums.topic.vote" => UserCode::User,
        "forums.topic.getVoteUsers" => UserCode::Admin,
        "forum.post.create" => {
            if anonym_allowed {
                UserCode::Anonym
            } else {
                UserCode::User
            }
        }
        "forum.post.update" => UserCode::User,
        "forums.post.delete" => UserCode::User,
        "like.create" => UserCode::User,
        "like.delete" => UserCode::User,
        "like.getUsers" => UserCode::Admin,
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
