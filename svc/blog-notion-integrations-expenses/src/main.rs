use notion::ids::{BlockId, DatabaseId, PageId};
use notion::models::paging::Paging;
use notion::models::search::{DatabaseQuery, FilterCondition, PropertyCondition, SelectCondition};
use notion::models::{Block, ListResponse, Page};
use notion::NotionApi;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use std::str::FromStr;
use tokio;
use regex::Regex;

async fn parse_and_update_transaction_in_notion(
    notion_token: &str,
    content: String,
    page_id: String,
    page_title: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let merchant: String;
    let amount: f64;
    let json: Value;

    let re =
        Regex::new(r"(\w+): You've spent (.+) (\d*\,?\d*.\d+) on your (\w+) card .* at (.*)\s*on")
            .unwrap();

    if let Some(caps) = re.captures(&content) {
        let amount_string = caps
            .get(3)
            .unwrap()
            .as_str()
            .trim()
            .to_string()
            .replace(',', "");
        merchant = caps.get(5).unwrap().as_str().trim().to_string();
        amount = amount_string.parse::<f64>().unwrap();
        let datetime = chrono::DateTime::parse_from_rfc2822(page_title.clone().as_str())
            .expect("Failed to parse date");

        // Format the DateTime object into ISO 8601 string
        let iso8601 = datetime.to_rfc3339();

        json = json!({
            "properties": {
                "ParseResult": {
                    "select": {
                        "name": "ParsedV1"
                    }
                },
                "Merchant": {
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": merchant,
                            }
                        }
                    ]
                },
                "Amount": {
                    "number": amount
                },
                "TransactionDate": {
                    "type": "date",
                    "date": {
                        "start": iso8601,
                    }
                },
            }
        });
    } else {
        json = json!({
            "properties": {
                "ParseResult": { "select": { "name": "FailedV1" } },
            }
        });
    }

    update_transaction_in_notion(notion_token, json, page_id).await?;

    Ok(())
}

async fn update_transaction_in_notion(
    notion_token: &str,
    json: Value,
    page_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HeaderMap::new();
    map.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {0}", notion_token))?,
    );

    map.insert("Notion-Version", "2022-06-28".parse().unwrap());

    let client = reqwest::Client::new();

    let resp = client
        .patch(format!("https://api.notion.com/v1/pages/{0}", page_id))
        .headers(map)
        .json(&json)
        .send()
        .await?;

    println!("Updated Transaction {:?}", resp.status());

    Ok(())
}

async fn search_database_items(notion_api: NotionApi, db_id: String, notion_token: &str) {
    let db_id: DatabaseId = DatabaseId::from_str(&db_id).expect("Can't parse to DatabaseId");

    let db_query = DatabaseQuery {
        sorts: None,
        filter: Some(FilterCondition {
            property: "ParseResult".to_string(),
            condition: PropertyCondition::Select(SelectCondition::IsEmpty),
        }),
        paging: Some(Paging {
            start_cursor: None,
            page_size: Some(u8::from_str_radix("1", 10).expect("Invalid u8")),
        }),
    };

    match notion_api.query_database(db_id, db_query).await {
        Ok(ListResponse::<Page> {
            results: response, has_more, ..
        }) => {
            println!("Finding transactions to parse...");
            for page in response {
                let page_title = page.title().expect("cannot get page title");
                println!("{}", page_title);
                let block_id = BlockId::from(page.id.clone());

                let blocks = notion_api.get_block_children(block_id).await;
                match blocks {
                    Ok(ListResponse::<Block> { results, .. }) => match results.into_iter().nth(0) {
                        None => {}
                        Some(Block::Paragraph { paragraph, .. }) => {
                            let content = paragraph
                                .rich_text
                                .iter()
                                .map(|rich_text| rich_text.plain_text())
                                .collect::<Vec<&str>>()
                                .join(" ");

                            let page_id = PageId::from(page.id.clone());

                            parse_and_update_transaction_in_notion(
                                notion_token,
                                content,
                                page_id.clone().to_string(),
                                page_title.clone().to_string(),
                            )
                            .await
                            .expect("Unable to write updates to Notion");
                        }
                        _ => todo!(),
                    },
                    Err(e) => {
                        eprintln!("Error reading page: {:?}", e);
                    }
                }
            }

            if has_more == false {
                println!("All transactions parsed.");
                std::process::exit(1)
            }
        }
        Err(e) => {
            eprintln!("Error searching pages: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    // Create a new NotionApi instance with your API key
    let notion_token =
        std::env::var("NOTION_INTEGRATION_TOKEN").expect("NOTION_INTEGRATION_TOKEN must be set");

    let db_id = std::env::var("NOTION_DB_ID").expect("NOTION_DB_ID must be set");

    match NotionApi::new(notion_token.clone()) {
        Ok(notion_api) => {
            search_database_items(notion_api, db_id, &notion_token.clone()).await;
        }
        Err(e) => {
            eprintln!("Error creating NotionApi instance {:?}", e);
        }
    }
}
