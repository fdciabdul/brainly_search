use reqwest;
use crate::models::{SearchPayload, ResponseData, TransformedAnswer, Query, Context, Pagination, TransformedAuthor};
use crate::utils::build_headers;

mod models;
mod utils;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let headers = build_headers();

    let payload = SearchPayload {
        query: Query {
            text: "guru".to_string(),
        },
        context: Context {
            supported_types: vec!["question"],
        },
        pagination: Pagination {
            cursor: None,
            limit: 20,
        },
    };

    let response_data: ResponseData = client
    .post("https://srv-unified-search.external.search-systems-production.z-dn.net/api/v1/id/search")
    .headers(headers)
    .json(&payload)
    .send()
    .await?
    .json::<ResponseData>()
    .await?;

    let transformed_data: Vec<TransformedAnswer> = response_data.results
    .into_iter()
    .map(|q| {
        let answer = q.question.answer;
        TransformedAnswer {
            id: answer.id.to_string(),
            author: TransformedAuthor {
                id: answer.author.id.to_string(),
                nick: answer.author.nick,
                rank: answer.author.rank,
            },
            content: answer.content.replace("<p>", "").replace("</p>", "\n").trim().to_string(),
            rates_count: answer.rates_count,  // Updated reference
            rating: answer.rating,
            thanks_count: answer.thanks_count, // Updated reference
        }
    })
    .collect();
    
    println!("{:?}", transformed_data);

    Ok(())
}
