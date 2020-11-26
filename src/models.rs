use serde::{Serialize, Deserialize};
use crate::schema::customers;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct CustomerDTO {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
}

#[derive(Debug, Clone, Deserialize, Insertable, AsChangeset)]
#[table_name = "customers"]
pub struct InsertableCustomerDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
}
