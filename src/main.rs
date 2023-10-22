use reqwest;
use clap::{Arg, App};
use std::fs::File;
use std::io::Write;
use crate::models::{SearchPayload, ResponseData, TransformedAnswer, Query, Context, Pagination, TransformedAuthor,TransformedQuestion};
use crate::utils::{html_to_text,build_headers,truncate_string};
use prettytable::{Table, row};
use term_size;

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
        let mut table = Table::new();
        let width = term_size::dimensions().map_or(80, |(w, _)| w);
        let max_col_width = (width - 10) / 2;
        table.add_row(row!["Pertanyaan", "Jawaban"]);

        for item in &transformed_data {
            let question_text = html_to_text(&item.content);
            let truncated_question = truncate_string(&question_text, max_col_width);
            if let Some(answer) = item.answers.first() {
                let answer_text = html_to_text(&answer.content);
                let truncated_answer = truncate_string(&answer_text, max_col_width);
                table.add_row(row![truncated_question, truncated_answer]);
            } else {
                table.add_row(row![truncated_question, "No Answer"]);
            }
        }
        
        table.printstd();

        
    }

    Ok(())
}