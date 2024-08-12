use notion::ids::{AsIdentifier, BlockId, DatabaseId};
use notion::NotionApi;
use notion::models::search::{DatabaseQuery, FilterProperty, FilterValue, NotionSearch};
use notion::models::{Block, ListResponse, Object, Page};
use tokio;
use std::str::FromStr; 

async fn search_database_items(notion_api: NotionApi,) {
    let db_id: DatabaseId = DatabaseId::from_str("9bd6d8e77ccb4a7c9296fb4382a0092a")
        .expect("Can't parse to DatabaseId");

    let db_query = DatabaseQuery {
        sorts: None,
        filter: None,
        paging: None
    };

    match notion_api.query_database(db_id, db_query).await {
        // Ok(ListResponse::<Block> { results: response, .. }) => {
        //     println!("Finding Notion Pages");
        //     for block in response {
        //         let page_title = page.title().expect("cannot get page title");
        //         println!("{:?}", page);
        //         println!("{}", page_title);

        //         let block = TryInto::<Block>::try_into(page);
        //         // page.try_into::<Block>();
        //         println!("{:?}", block);

        //     }
        // },
        Ok(ListResponse::<Page> { results: response, .. }) => {
            println!("Finding Notion Pages");
            for page in response {
                println!("_____________");
                let page_title = page.title().expect("cannot get page title");
                // println!("{:?}", page);
                println!("{}", page_title);

                // let block = TryInto::<Block>::try_into(page);
                // page.try_into::<Block>();
                // println!("{:?}", block);
                let block_id = BlockId::from(page.id);

                let blocks = notion_api.get_block_children(block_id).await;
                match blocks {
                    Ok(ListResponse::<Block> { results, ..}) => {
                        match results.into_iter().nth(0) {
                            None => {},
                            Some(Block::Paragraph { paragraph, .. }) => {
                                let content = paragraph.rich_text
                                    .iter()
                                    .map(|rich_text| rich_text.plain_text())
                                    .collect::<Vec<&str>>()
                                    .join(" ");

                                println!("{:?}", content);
                            }
                            _ => todo!()
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading page: {:?}", e);
                    }
                }

            }
        }
        Err(e) => {
            eprintln!("Error searching pages: {:?}", e);
        }
    }
}

async fn _search_databases(notion_api: NotionApi) {
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

#[tokio::main]
async fn main() {
    // Create a new NotionApi instance with your API key
    let notion_token = std::env::var("NOTION_INTEGRATION_TOKEN")
        .expect("NOTION_INTEGRATION_TOKEN must be set");

    match NotionApi::new(notion_token) {
        Ok(notion_api) => {
            // search_databases(notion_api).await
            search_database_items(notion_api).await
        },
        Err(e) => { eprintln!("Error creating NotionApi instance {:?}", e); }
    }
}
