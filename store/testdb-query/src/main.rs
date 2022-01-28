use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;

fn search(
    conn: &PgConnection,
    title: Option<String>,
    body: Option<String>,
) -> Result<Vec<Post>, diesel::result::Error> {
    use diesel_sample::schema::posts;
    let query = posts::table.order(posts::id.desc());

    query = title
        .map(|x| query.filter(posts::title.eq(&x)))
        .unwrap_or(query);
    query = body
        .map(|x| query.filter(posts::body.eq(&x)))
        .unwrap_or(query);

    query.load::<Post>(conn)
}

fn main() {
    let seans_id = users.filter(name.eq("Sean")).select(id)
        .first(&connection);
    assert_eq!(Ok(1), seans_id);
    let tess_id = users.filter(name.eq("Tess")).select(id)
        .first(&connection);
    assert_eq!(Ok(2), tess_id);
}
