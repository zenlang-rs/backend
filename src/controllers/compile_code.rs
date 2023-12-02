use axum::{extract, Json};
use serde::{Deserialize, Serialize};
use zen::run_program;

#[derive(Deserialize)]
pub struct CodeCompileRequest {
    pub code: String,
}
#[derive(Serialize)]
pub struct CodeOutputResponse {
    pub output: Result<String, String>,
}

// #[derive(Deserialize)]
// struct CodeQuizRequest {
//     pub code: String,
//     pub testcases: Vec<Testcase>
// }
//
// struct Testcase {
//     pub input: String,
//     pub expected_output: String
// }
//
// #[derive(Serialize)]
// struct QuizResponse {
//     pub output_match: Vec<Result<bool, String>>
// }


fn runnable_code(code: String, input: &str) -> Result<String, String> {
    let runnable = run_program(code, input);
    match runnable {
        Ok(output) => Ok(output),
        Err(err) => Err(format!("[ERROR]\n{}\n{}", err.msg, err.error_type)),
    }
}

pub async fn compile_code(
    extract::Json(user): extract::Json<CodeCompileRequest>,
) -> Json<CodeOutputResponse> {
    Json(CodeOutputResponse {
        output: runnable_code(user.code, ""),
    })
}

// async fn take_quiz(
//     extract::Json(user): extract::Json<CodeQuizRequest>,
// ) -> Json<QuizResponse> {
//     let output_match =
//     Json(QuizResponse {
//         output_match: ,
//     })
// }
//
// fn match_outputs(code: String, testcases: Vec<Testcase>) -> Vec<Result<bool, String>> {
//     for testcase in testcases.iter() {
//         runnable_code(code, testcase.input)
//     }
//     vec![]
// }