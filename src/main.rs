use reqwest;
use clap::{Arg, App};
use std::fs::File;
use std::io::Write;
use crate::models::{SearchPayload, ResponseData, TransformedAnswer, Query, Context, Pagination, TransformedAuthor,TransformedQuestion};
use crate::utils::build_headers;

mod models;
mod utils;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Brainly Search")
        .version("0.1.0")
        .about("Searches Brainly for questions.")
        .arg(Arg::with_name("question")
            .short("q")
            .long("question")
            .value_name("QUESTION")
            .help("Sets the question keyword to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Sets the output file for search results")
            .takes_value(true))
        .get_matches();

    let query_text = matches.value_of("question").unwrap().to_string();

    let client = reqwest::Client::new();
    let headers = build_headers();

    let payload = SearchPayload {
        query: Query {
            text: query_text,
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

    let transformed_data: Vec<TransformedQuestion> = response_data.results
    .into_iter()
    .map(|q| {
        let answer = &q.question.answer;
        let transformed_answer = TransformedAnswer {
            id: answer.id.to_string(),
            author: TransformedAuthor {
                id: answer.author.id.to_string(),
                nick: answer.author.nick.clone(),
                rank: answer.author.rank.clone(),
            },
            content: answer.content.replace("<p>", "").replace("</p>", "\n").trim().to_string(),
            rates_count: answer.rates_count,
            rating: answer.rating,
            thanks_count: answer.thanks_count,
        };

        TransformedQuestion {
            question_id: q.question.id.to_string(),
            content: q.question.content.replace("<br />", "\n"),
            answer_count: q.question.answer_count,
            subject_id: q.question.subject_id,
            answers: vec![transformed_answer],
        }
    })
    .collect();

    let json_output = serde_json::to_string_pretty(&transformed_data)?;

    if let Some(output_path) = matches.value_of("output") {
        let mut file = File::create(output_path)?;
        writeln!(file, "{}", json_output)?;
    } else {
        println!("{}", json_output);
    }

    Ok(())
}