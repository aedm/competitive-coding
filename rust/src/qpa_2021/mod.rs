use crate::utils::read_scratch_file;
use anyhow::Result;
use reqwest::Method;
use serde_json::{json, to_string, to_string_pretty, Value};
use std::time::Instant;

pub mod qpa2021_1;
mod qpa2021_2;
mod qpa2021_3;
mod qpa2021_4;
mod qpa2022_2;
mod qpa2022_3;
mod qpa2022_4;
mod qpa2022_5;

pub async fn qpa_main() -> Result<()> {
    // let problem = ("barrel", qpa2021_4::solve);
    let problem = ("saw-squarey", qpa2022_5::solve);

    let input = serde_json::from_str(&read_scratch_file()).unwrap();
    let output = problem.1(&input);
    println!("Output:\n{}", to_string(&output)?);
    return Ok(());

    let submission: Value = call_api(
        Method::POST,
        "/api/submissions/start-submission",
        &json!({
            "problem": problem.0,
            // "sample_index": 0
        }),
    )
    .await;

    let test_count = submission["submission"]["test_count"].as_i64().unwrap();
    let submission_id = submission["submission"]["id"].as_str().unwrap();
    println!("Submission: {} {} {}", problem.0, test_count, submission_id);

    let mut correct_count = 0;
    for i in 1..=test_count {
        println!("Solving {}/{}", i, test_count);

        let test_case = call_api(
            Method::PUT,
            "/api/submissions/test",
            &json!({ "submission": submission_id }),
        )
        .await;

        let input = &test_case["input"];
        let test_id = test_case["test_id"].as_str().unwrap();
        println!("Input:\n{}", to_string(&input)?);
        let output = problem.1(&input);
        println!("Output:\n{}", to_string_pretty(&output)?);

        let result = call_api(
            Method::POST,
            &format!("/api/submissions/test/{}", test_id),
            &json!({ "output": output }),
        )
        .await;

        let is_correct = result["correct"].as_bool().unwrap();
        if is_correct {
            correct_count += 1;
        }
        println!("Result: {}\n---", is_correct);
    }

    println!(
        "Correct solutions for '{}': {} / {}",
        problem.0, correct_count, test_count
    );
    Ok(())
}

async fn call_api(method: Method, url: &str, body: &Value) -> Value {
    reqwest::Client::new()
        .request(method, format!("https://challenge.snapsoft.hu{}", url))
        .header("X-Api-Token", std::env::var("QPA_API_TOKEN").unwrap())
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap()
}
