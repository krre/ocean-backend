use crate::controller::*;
use crate::types::Id;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::{Int4, Int8, Text};
use serde::{Deserialize, Serialize};

#[derive(QueryableByName, Serialize)]
pub struct Section {
    #[sql_type = "Int4"]
    id: Id,
    #[sql_type = "Text"]
    name: String,
    #[sql_type = "Int4"]
    category_id: Id,
    #[sql_type = "Int8"]
    topic_count: i64,
    #[sql_type = "Int8"]
    post_count: i64,
}

pub fn get_sections(
    db: &db::Db,
    category_id: Option<Id>,
) -> Result<Vec<Section>, Box<dyn std::error::Error>> {
    let mut result = diesel::dsl::sql_query(
        "SELECT fs.id, fs.name, fs.category_id,
	        (SELECT COUNT(*) FROM forum_topics WHERE section_id = fs.id) AS topic_count,
	        (SELECT COUNT(*) FROM forum_posts AS fp
		        JOIN forum_topics AS ft ON ft.id = fp.topic_id
		WHERE ft.section_id = fs.id) AS post_count
        FROM forum_sections AS fs
        ORDER BY fs.order_index ASC",
    )
    .load::<Section>(&db.conn)?;

    if let Some(id) = category_id {
        // Expensive but simple
        result = result
            .into_iter()
            .filter(|section| section.category_id == id)
            .collect();
    }

    Ok(result)
}

// forum.section.getAll
pub fn get_all(data: RequestData) -> RequestResult {
    #[derive(Deserialize)]
    struct Req {
        category_id: Id,
    }

    let req: Req = data.params()?;

    use crate::model::schema::forum_categories;

    let category_name = forum_categories::table
        .select(forum_categories::name)
        .filter(forum_categories::id.eq(req.category_id))
        .first::<String>(&data.db.conn)?;

    let sections = get_sections(&data.db, Some(req.category_id))?;

    #[derive(Serialize)]
    struct Resp {
        category_name: String,
        sections: Vec<Section>,
    }

    let resp = Resp {
        category_name,
        sections,
    };

    let result = serde_json::to_value(&resp)?;
    Ok(Some(result))
}

// forum.section.getOne
pub fn get_one(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections::dsl::*;

    let req: RequestId = data.params()?;

    #[derive(Queryable, Serialize)]
    pub struct ForumSection {
        category_id: Id,
        name: String,
        order_index: i16,
    }

    let forum_section = forum_sections
        .select((category_id, name, order_index))
        .filter(id.eq(req.id))
        .first::<ForumSection>(&data.db.conn)
        .optional()?;

    let result = serde_json::to_value(&forum_section)?;
    Ok(Some(result))
}

// forum.section.create
pub fn create(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Insertable, Deserialize)]
    #[table_name = "forum_sections"]
    struct Req {
        category_id: Id,
        name: String,
        order_index: i16,
    }

    let req: Req = data.params()?;

    diesel::insert_into(forum_sections)
        .values(&req)
        .execute(&data.db.conn)?;

    Ok(None)
}

// forum.section.update
pub fn update(data: RequestData) -> RequestResult {
    use crate::model::schema::forum_sections;
    use crate::model::schema::forum_sections::dsl::*;

    #[derive(Deserialize)]
    struct Req {
        id: Id,
        name: String,
        order_index: i16,
    }

    let req: Req = data.params()?;

    #[derive(AsChangeset)]
    #[table_name = "forum_sections"]
    pub struct UpdateForumSection {
        pub name: String,
        pub order_index: i16,
        pub update_ts: NaiveDateTime,
    }

    let update_forum_section = UpdateForumSection {
        name: req.name,
        order_index: req.order_index,
        update_ts: Utc::now().naive_utc(),
    };

    diesel::update(forum_sections.filter(id.eq(req.id)))
        .set(&update_forum_section)
        .execute(&data.db.conn)?;

    Ok(None)
}

// forum.section.delete
pub fn delete(data: RequestData) -> RequestResult {
    let req: RequestId = data.params()?;

    use crate::model::schema::forum_sections::dsl::*;

    diesel::delete(forum_sections.filter(id.eq(req.id))).execute(&data.db.conn)?;
    Ok(None)
}
