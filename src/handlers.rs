use actix_web::{
    web::{Data, Path, Json},
    get, post, put, delete,
    error::{self, Error},
};
use diesel::NotFound;

use crate::models::{CustomerDTO, InsertableCustomerDTO};
use crate::data_access::DBAccessManager;

#[get("/customers")]
pub async fn customers_list(db_manager: Data<DBAccessManager>) -> Result<Json<Vec<CustomerDTO>>, Error> {
    log::info!("handling list of customers");

    let result = db_manager.list_customers().await;

    result
        .map(Json)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))
}

#[post("/customers")]
pub async fn create_customer(db_manager: Data<DBAccessManager>, new_customer: Json<InsertableCustomerDTO>) -> Result<Json<CustomerDTO>, Error> {
    log::info!("handling add customer");

    let dto = db_manager.create_customer(new_customer.clone()).await;

    dto
        .map(Json)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))
}

#[get("/customers/{id}")]
pub async fn get_customer(customer_id: Path<i64>, db_manager: Data<DBAccessManager>) -> Result<Json<CustomerDTO>, Error> {
    log::info!("handling delete customer");

    let result = db_manager.get_customer(*customer_id).await;

    result
        .map(Json)
        .map_err(|e| match e {
            NotFound => error::ErrorNotFound("Not Found"),
            e => error::ErrorInternalServerError(e.to_string()),
        })
}

#[put("/customers/{id}")]
pub async fn update_customer(customer_id: Path<i64>, db_manager: Data<DBAccessManager>, updated_customer: Json<InsertableCustomerDTO>) -> Result<Json<usize>, Error> {
    log::info!("handling update customer");

    let response = db_manager.update_customer(*customer_id, updated_customer.clone()).await;

    response
        .map(Json)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))
}

#[delete("/customers/{id}")]
pub async fn delete_customer(customer_id: Path<i64>, db_manager: Data<DBAccessManager>) -> Result<Json<usize>, Error> {
    log::info!("handling delete customer");

    let result = db_manager.delete_customer(*customer_id).await;

    result
        .map(Json)
        .map_err(|e| match e {
            NotFound => error::ErrorNotFound("Not Found"),
            e => error::ErrorInternalServerError(e.to_string()),
        })
}
