use reqwasm::http;
use common::{User, ErrorResponse, UserData, UserLogin, UserResponse};

pub async fn api_create_user(user_data: &str) -> Result<User, String> {
    let response = match http::Request::post("https://www.shebolaheh.com/api/add_user")
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserData>().await;
    match res_json {
        Ok(data) => Ok(data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_fetch_user(user_id: &str) -> Result<User, String> {
    let response = match http::Request::get(
        format!("https://www.shebolaheh.com/api/get_user/{}", user_id).as_str(),
    )
    .send()
    .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<UserResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.user),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
