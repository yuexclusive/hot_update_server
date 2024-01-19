use crate::model::user as user_model;
use meilisearch_sdk::search::Selectors;
use util_error::BasicResult;
use util_response::Pagination;

pub async fn search(
    key_word: &str,
    page: &Pagination,
) -> BasicResult<(Vec<user_model::SearchedUser>, usize)> {
    let res = util_meilisearch::client()
        .get_index(super::USER_LIST_INDEX)
        .await?
        .search()
        .with_sort(&["updated_at:desc"])
        .with_attributes_to_highlight(Selectors::Some(&[
            "email",
            "type",
            "status",
            "name",
            "mobile",
            "laston",
            "created_at",
            "updated_at",
        ]))
        .with_highlight_pre_tag("<span class=\"highlight\">")
        .with_highlight_post_tag("</span>")
        .with_query(&key_word)
        .with_offset(page.skip() as usize)
        .with_limit(page.take() as usize)
        .execute::<user_model::User>()
        .await?;

    let data = res
        .hits
        .into_iter()
        .map(|x| user_model::SearchedUser {
            formatter: x.formatted_result.unwrap().into(),
            user: x.result,
        })
        .collect::<Vec<user_model::SearchedUser>>();

    Ok((data, res.estimated_total_hits.unwrap_or_default()))
}
