use axum::{extract::Path, http::StatusCode, response::Json, Extension};
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::info;

use rust_velocimeter_data::database::o_token_datas::Entity as OTokenDatas;
use rust_velocimeter_data::database::o_token_datas::Model as OTokenData;

use super::internal_error;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

#[axum_macros::debug_handler]
pub async fn give_options_data(
    Path(chain_id): Path<i32>,
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Json<OTokenData>, StatusCode> {
    info!("chain_id: {}", chain_id);
    let res = OTokenDatas::find_by_id(chain_id)
        .one(&conn)
        .await
        .map_err(internal_error)?;

    match res {
        Some(data) => Ok(Json(data.into())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
