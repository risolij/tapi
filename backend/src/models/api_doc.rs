use utoipa::OpenApi;

use crate::models::transaction::{
    PostTransaction, 
    Transaction, 
    UpdateTransaction
};

#[derive(OpenApi)]
#[openapi(
    handlers(
        crate::controllers::transaction::get_transactions,
        crate::controllers::transaction::post_transaction,
        crate::controllers::transaction::get_transaction_by_id,
        crate::controllers::transaction::update_transaction,
        crate::controllers::transaction::delete_transaction
    ),
    components(Transaction, PostTransaction, UpdateTransaction)
)]
pub struct ApiDoc;
