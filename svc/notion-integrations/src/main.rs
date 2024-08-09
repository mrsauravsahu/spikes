use notion::NotionApi;
use notion::models::search::{NotionSearch,FilterValue,FilterProperty};
use notion::models::{Object,ListResponse};
use tokio;

async fn search_databases(notion_api: NotionApi) {
    // Call the search method to list all databases
    let search_request = NotionSearch::Filter {
        property: FilterProperty::Object,
        value: FilterValue::Database
    };

    match notion_api.search(search_request).await {
        Ok(ListResponse::<Object> { results: response, .. }) => {
            println!("Finding Notion Databases");
            for obj in response {
                match obj {
                    Object::Database { database } => {

                        let joined_plain_texts: String = database
                            .title
                            .iter()
                            .map(|rich_text| rich_text.plain_text())
                            .collect::<Vec<&str>>()
                            .join(" ");

                        println!("{}", joined_plain_texts);
                    },
                    _ => {
                        println!("The object is not a Database");
                    },
                }
            }
        }
        Err(e) => {
            eprintln!("Error searching databases: {:?}", e);
        }
    }
}

fn main() {
    // Create a new NotionApi instance with your API key
    let notion_token = std::env::var("NOTION_INTEGRATION_TOKEN").expect("NOTION_INTEGRATION_TOKEN must be set");

    match NotionApi::new(notion_token) {
        Ok(notion_api) => {
            tokio::runtime::Runtime::new().unwrap().block_on(search_databases(notion_api));
        },
        Err(e) => { eprintln!("Error creating NotionApi instance {:?}", e); }
    }
}
