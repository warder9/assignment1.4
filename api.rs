use actix_web::{web, HttpResponse};
use serde::Deserialize;
use reqwest;
use serde_json::Value;

#[derive(Deserialize)]
struct Query {
    query: String,
}

pub async fn search_news(query: web::Path<String>) -> HttpResponse {
    let query = query.into_inner();

    let newsdata_api_key = "pub_8065342e0935ad9a609d047f9cbf926202bb5";
    let url = format!(
        "https://newsdata.io/api/1/news?apikey={}&q={}&category=business&language=en",
        newsdata_api_key,
        query
    );

    let res = reqwest::get(&url).await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body = response.text().await.unwrap_or("No data.".into());

                let parsed: Result<Value, _> = serde_json::from_str(&body);
                if let Ok(json) = parsed {
                    if let Some(results) = json.get("results").and_then(|v| v.as_array()) {
                        let mut html = String::from("<h1>Результаты по запросу:</h1><ul>");

                        for article in results {
                            if let (Some(title), Some(link)) = (
                                article.get("title").and_then(|v| v.as_str()),
                                article.get("link").and_then(|v| v.as_str()),
                            ) {
                                html.push_str(&format!(
                                    "<li><a href=\"{}\" target=\"_blank\">{}</a></li>",
                                    link, title
                                ));
                            }
                        }

                        html.push_str("</ul>");
                        return HttpResponse::Ok()
                            .content_type("text/html; charset=utf-8")
                            .body(html);
                    }
                }

                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body("Не удалось распарсить JSON или нет результатов.")
            } else {
                HttpResponse::InternalServerError()
                    .body(format!("Ошибка: {}", response.status()))
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Не удалось получить новости."),
    }
}
