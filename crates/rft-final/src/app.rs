use crate::{
    common::config,
    domain::{
        self,
        shared::db::{Filter, FilterValue, Pagination, Query, Sort, SortOrder},
    },
    infra,
};

pub struct App {}

impl App {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub async fn init() -> Result<App, Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let cfg = config::Config::new()?;

    let _tracing = infra::tracing::init(&cfg.tracing);

    let db = infra::db::connect(&cfg.database).await?;

    let _user_repository = domain::user::UserRepository::new(db);

    let users = _user_repository
        .find(Query {
            pagination: Some(Pagination {
                page: 1,
                page_size: 10,
            }),
            search: None,
            sorts: Some(vec![Sort {
                by: "name".to_string(),
                order: SortOrder::Asc,
            }]),
            filters: Some(vec![Filter {
                by: "is_active".to_string(),
                value: FilterValue::Boolean(false),
            }]),
        })
        .await
        .unwrap();

    println!("{:?}", users);

    Ok(App {})
}
