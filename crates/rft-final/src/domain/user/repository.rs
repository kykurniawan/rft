use sqlx::{Pool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::domain::{
    shared::{
        db::{Paginated, Pagination, Query, SortOrder},
        error::RepositoryError,
    },
    user::User,
};

#[derive(Clone)]
pub struct UserRepository {
    db: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn find(&self, query: Query) -> Result<Paginated<User>, RepositoryError> {
        let pagination = query.pagination.clone().unwrap_or(Pagination {
            page: 1,
            page_size: 10,
        });

        let offset = (pagination.page - 1) * pagination.page_size;

        let mut count_builder =
            QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM users WHERE 1 = 1");

        self.apply_user_conditions(&mut count_builder, &query);

        let total: i64 = count_builder
            .build_query_scalar()
            .fetch_one(&self.db)
            .await?;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            SELECT id, name, is_active, created_at, updated_at FROM users WHERE 1 = 1
            "#,
        );

        self.apply_user_conditions(&mut builder, &query);

        if let Some(sorts) = &query.sorts {
            let mut has_order_by = false;

            for sort in sorts {
                match sort.by.as_str() {
                    "name" => {
                        if !has_order_by {
                            builder.push(" ORDER BY ");
                            has_order_by = true;
                        } else {
                            builder.push(", ");
                        }

                        // Do NOT use push_bind for column names
                        builder.push("name");

                        match sort.order {
                            SortOrder::Asc => builder.push(" ASC"),
                            SortOrder::Desc => builder.push(" DESC"),
                        };
                    }

                    "created_at" => {
                        if !has_order_by {
                            builder.push(" ORDER BY ");
                            has_order_by = true;
                        } else {
                            builder.push(", ");
                        }

                        builder.push("created_at");

                        match sort.order {
                            SortOrder::Asc => builder.push(" ASC"),
                            SortOrder::Desc => builder.push(" DESC"),
                        };
                    }

                    _ => {}
                }
            }

            if !has_order_by {
                builder.push(" ORDER BY name ASC");
            }
        } else {
            builder.push(" ORDER BY name ASC");
        }

        builder
            .push(" LIMIT ")
            .push_bind(pagination.page_size)
            .push(" OFFSET ")
            .push_bind(offset);

        let users = builder.build_query_as::<User>().fetch_all(&self.db).await?;

        Ok(Paginated {
            items: users,
            total,
            page: pagination.page,
            page_size: pagination.page_size,
        })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as(
            "SELECT id, name, is_active, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    fn apply_user_conditions(&self, builder: &mut QueryBuilder<Postgres>, query: &Query) {
        if let Some(filters) = &query.filters {
            for filter in filters {
                match filter.by.as_str() {
                    "is_active" => {
                        let value = match filter.value.as_str() {
                            "true" => true,
                            "false" => false,
                            _ => continue,
                        };
                        builder.push(" AND is_active = ").push_bind(value);
                    }

                    _ => {}
                }
            }
        }

        if let Some(search) = &query.search {
            let search = format!("%{}%", search);
            builder.push(" AND name ILIKE ").push_bind(search);
        }
    }
}
