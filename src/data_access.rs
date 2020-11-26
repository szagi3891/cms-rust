use diesel::prelude::*;

use crate::AsyncPool;
use crate::models::{CustomerDTO, InsertableCustomerDTO};

#[derive(Clone)]
pub struct DBAccessManager {
    pool: AsyncPool,
}

impl DBAccessManager {
    pub fn new(pool: AsyncPool) -> DBAccessManager {
        DBAccessManager {
            pool
        }
    }

    pub async fn list_customers(&self) -> QueryResult<Vec<CustomerDTO>> {
        use super::schema::customers::dsl::*;

        self.pool.get(|connection|
            customers.load(connection)
        ).await
    }

    pub async fn create_customer(&self, dto: InsertableCustomerDTO) -> QueryResult<CustomerDTO> {
        use super::schema::customers;

        self.pool.get(move |connection|
            diesel::insert_into(customers::table) // insert into customers table
                .values(&dto) // use values from CreateCustomerDTO
                .returning(customers::all_columns)
                .get_result(connection) // execute query
        ).await
    }

    pub async fn update_customer(&self, customer_id: i64, updated_customer: InsertableCustomerDTO) -> QueryResult<usize> {
        use super::schema::customers::dsl::*;

        self.pool.get(move |connection| {
            let updated = diesel::update(customers)
                .filter(id.eq(customer_id))
                .set(updated_customer)
                .execute(connection)?;

            if updated == 0 {
                return Err(diesel::NotFound)
            }
            Ok(updated)
        }).await
    }

    pub async fn delete_customer(&self, customer_id: i64) -> QueryResult<usize> {
        use super::schema::customers::dsl::*;

        self.pool.get(move |connection| {
            let deleted = diesel::delete(customers.filter(id.eq(customer_id)))
                .execute(connection)?;

            if deleted == 0 {
                return Err(diesel::NotFound)
            }
            Ok(deleted)
        }).await
    }

    pub async fn get_customer(&self, customer_id: i64) -> QueryResult<CustomerDTO> {
        use super::schema::customers::dsl::*;

        self.pool.get(move |connection|
            customers.find(customer_id)
                .get_result(connection)
        ).await
    }
}
