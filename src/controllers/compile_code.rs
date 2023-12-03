use axum::{extract, Json};
use serde::{Deserialize, Serialize};
use zen::run_program;

#[derive(Deserialize)]
pub struct CodeCompileRequest {
    pub code: String,
    pub input: String,
}
#[derive(Serialize)]
pub struct CodeOutputResponse {
    pub output: Result<String, String>,
}

#[derive(Deserialize)]
pub struct CodeQuizRequest {
    pub code: String,
    pub testcases: Vec<Testcase>,
}

#[derive(Deserialize)]
pub struct Testcase {
    pub input: String,
    pub expected_output: String,
}

#[derive(Serialize)]
pub struct QuizResponse {
    pub output_match: Vec<Result<bool, String>>,
}

pub async fn compile_code(
    extract::Json(user): extract::Json<CodeCompileRequest>,
) -> Json<CodeOutputResponse> {
    Json(CodeOutputResponse {
        output: runnable_code(user.code, &user.input),
    })
}

pub async fn take_quiz(extract::Json(user): extract::Json<CodeQuizRequest>) -> Json<QuizResponse> {
    Json(QuizResponse {
        output_match: match_outputs(user.code, user.testcases),
    })
}

fn runnable_code(code: String, input: &str) -> Result<String, String> {
    let runnable = run_program(code, input, false);
    match runnable {
        Ok(output) => Ok(output),
        Err(err) => Err(format!("[ERROR]\n{}\n{}", err.msg, err.error_type)),
    }
}

fn match_outputs(code: String, testcases: Vec<Testcase>) -> Vec<Result<bool, String>> {
    let mut output_vec: Vec<Result<bool, String>> = vec![];

    for testcase in testcases.iter() {
        let code_instance = runnable_code(code.clone(), &testcase.input);

        match code_instance {
            Ok(actual_output) => {
                output_vec.push(Ok(actual_output.trim_end() == testcase.expected_output))
            }
            Err(err) => output_vec.push(Err(err)),
        }
    }
    output_vec
}
