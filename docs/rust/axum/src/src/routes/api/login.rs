/* #region modules */
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::Result;
/* #endregion */


/* #region export Route */
pub fn route() -> Router {
    Router::new().route("/login", post(login))
}
/* #endregion */


/* #region Handler */
async fn login(body: Json<LoginBody>) -> Result<Json<Value>> {
    let res_body = Json(json!({
        "resutl" : {
            "success" : true,
        }
    }));

    Ok(res_body)
}

#[derive(Deserialize)]
struct LoginBody {
}

/* #endregion */