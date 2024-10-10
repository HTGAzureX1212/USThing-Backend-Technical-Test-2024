use std::collections::HashMap;
use aide::axum::IntoApiResponse;
use axum::Json;
use backend_models::{LibraryOpeningHour, LibraryOpeningHourRequestBody, LibraryOpeningHourResponseEntry};
use reqwest::StatusCode;
use serde_json::json;

pub async fn library_recent_hours(
    Json(body): Json<LibraryOpeningHourRequestBody>,
) -> impl IntoApiResponse {
    if !["G/F Entrance", "Learning Commons", "LG5 Entrance", "all"].contains(&body.filter.as_str())
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": 400,
                "status": "Bad Request",
            })),
        );
    }

    let result_resp =
        reqwest::get("https://lbcone.hkust.edu.hk/hours/hoursapi/gethours?func=calendar").await;

    if result_resp.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "code": 500,
                "status": "Internal Server Error"
            })),
        );
    }

    let result_hours = result_resp.unwrap().json::<Vec<LibraryOpeningHour>>().await;
    if result_hours.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "code": 500,
                "status": "Internal Server Error"
            })),
        );
    }

    let hours = result_hours.unwrap();

    let ground = hours
        .iter()
        .cloned()
        .filter(|hour| hour.name == String::from("G/F Entrance"))
        .collect::<Vec<_>>();
    let lg1 = hours
        .iter()
        .cloned()
        .filter(|hour| hour.name == String::from("Learning Commons"))
        .collect::<Vec<_>>();
    let lg5 = hours
        .iter()
        .cloned()
        .filter(|hour| hour.name == String::from("LG5 Entrance"))
        .collect::<Vec<_>>();

    let result = match &*body.filter {
        "all" => {
            let map = ground.iter().take(body.days as usize)
                .enumerate()
                .map(|(i, hour)| (hour.start.clone(), vec![
                    LibraryOpeningHourResponseEntry::from(hour.clone()),
                    LibraryOpeningHourResponseEntry::from(lg1[i].clone()),
                    LibraryOpeningHourResponseEntry::from(lg5[i].clone()),
                ]))
                .collect::<HashMap<_, _>>();

            json!({
                "code": 200,
                "status": "OK",
                "data": map
            })
        }
        _ => unreachable!()
    };

    (StatusCode::OK, Json(result))
}
