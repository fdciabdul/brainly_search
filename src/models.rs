use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct SearchPayload {
    pub query: Query,
    pub context: Context,
    pub pagination: Pagination,
}

#[derive(Serialize)]
pub struct Query {
    pub text: String,
}

#[derive(Serialize)]
pub struct Context {
    #[serde(rename = "supportedTypes")]
    pub supported_types: Vec<&'static str>,
}

#[derive(Serialize)]
pub struct Pagination {
    pub cursor: Option<Value>,
    pub limit: i32,
}

#[derive(Debug, Deserialize)]
pub struct ResponseData {
    pub results: Vec<QuestionResult>,
}

#[derive(Debug, Deserialize)]
pub struct QuestionResult {
    pub question: QuestionData,
}

#[derive(Debug, Deserialize)]
pub struct QuestionData {
    pub id: i64,
    pub content: String,
    #[serde(rename = "answerCount")]
    pub answer_count: i32,
    #[serde(rename = "subjectId")]
    pub subject_id: Option<i32>, 
    pub answer: AnswerData,
}

#[derive(Debug, Deserialize)]
pub struct AnswerData {
    pub id: i64,
    pub author: AuthorData,
    pub content: String,
    #[serde(rename = "ratesCount")]
    pub rates_count: i32,
    pub rating: f32,
    #[serde(rename = "thanksCount")]
    pub thanks_count: i32,
}

#[derive(Debug, Deserialize)]
pub struct AuthorData {
    pub id: i64,
    pub nick: String,
    pub rank: String,
}

#[derive(Debug, Serialize)]
pub struct TransformedAnswer {
    pub id: String,
    pub author: TransformedAuthor,
    pub content: String,
    pub rates_count: i32,
    pub rating: f32,
    pub thanks_count: i32,
}

#[derive(Debug, Serialize)]
pub struct TransformedAuthor {
    pub id: String,
    pub nick: String,
    pub rank: String,
}

#[derive(Serialize)]
pub struct TransformedQuestion {
    pub question_id: String,
    pub content: String,
    pub answer_count: i32,
    #[serde(rename = "subjectId")]
    pub subject_id: Option<i32>,
    pub answers: Vec<TransformedAnswer>,
}