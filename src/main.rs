use serde::{Deserialize};
use serde_json::Value;
use std::{error::Error, f32::consts::E};
use reqwest::Client;
use tokio::runtime::Runtime;
use rocket_contrib::json::{Json};
use std::io::{stdin, stdout, Write};
use prettytable::{Table, Row, Cell};
use reqwest::StatusCode;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Book {
    id: u32,
    title: String,
    author: String,
    description: String,
}

async fn get_all_books() -> Result<Json<Value>, Box<dyn Error>> {
    let client = Client::new();
    let res = client
                        .get("https://rest-api-server-book.onrender.com/books")
                        .timeout(Duration::from_secs(5))
                        .send()
                        .await?;
    if res.status() == StatusCode::OK
    {
        let books = res.json::<serde_json::Value>().await?;
    
        Ok(Json(books))
    }
    else 
    {
        Err(format!("Failed to get books: {}", res.status()).into())
    }

}

async fn get_book(id: u32) -> Result<Json<Value>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://rest-api-server-book.onrender.com/books/{}", id);
    let res = client
                        .get(&url)
                        .timeout(Duration::from_secs(5))
                        .send()
                        .await?;
    if res.status() == StatusCode::OK
    {
        let book = res.json::<serde_json::Value>().await?;
        Ok(Json(book))
    }
    else 
    {
        Err(format!("Failed to get books: {}", res.status()).into())
    }
}

fn print_books_table(books_json: &Json<Value>) {
    let books_vec: Vec<Book> = books_json.as_array()
    .unwrap()
    .iter()
    .map(|book| serde_json::from_value::<Book>(book.clone()).unwrap())
    .collect::<Vec<Book>>();

    let mut table = Table::new();

    // Define table headers
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Title"),
        Cell::new("Author"),
        Cell::new("Description"),
    ]));

    // Populate table rows with book data
    for book in books_vec {
        table.add_row(Row::new(vec![
            Cell::new(&book.id.to_string()),
            Cell::new(&book.title),
            Cell::new(&book.author),
            Cell::new(&book.description),
        ]));
    }

    // Print table
    table.printstd();
}

fn print_book_table(book_json: &Json<Value>) {
    let book_value = book_json.as_object().unwrap();

    let mut table = Table::new();

    //Define table headers
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Title"),
        Cell::new("Author"),
        Cell::new("Description"),
    ]));

    // Populate table rows with book data
    table.add_row(Row::new(vec![
        Cell::new(&book_value["id"].to_string()),
        Cell::new(&book_value["title"].to_string()),
        Cell::new(&book_value["author"].to_string()),
        Cell::new(&book_value["description"].to_string()),
    ]));

    table.printstd();

}
fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        print!("Enter 'get' to get all books, number to get book id or exit to close the program: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "get" => {
                let books = Runtime::new()?.block_on(get_all_books())?;
                // println!("{:#?}", books);
                print_books_table(&books);
            }
            "1" => {
                let book = Runtime::new()?.block_on(get_book(1))?;
                // println!("{:#?}", book);
                print_book_table(&book);
                
            }
            "2" => {
                let book = Runtime::new()?.block_on(get_book(2))?;
                // println!("{:#?}", book);
                print_book_table(&book);
                
            }
            "3" => {
                let book = Runtime::new()?.block_on(get_book(3))?;
                // println!("{:#?}", book);
                print_book_table(&book);
                
            }
            "exit" => break,
            _ => println!("Invalid choice. Please try again."),
        }
        input.clear();
    }
    Ok(())
}

// #[tokio::main]
// async fn main() {
//     let client = Client::new();
//     let res = client.get("http://localhost:8000/books").send().await;
//     match res {
//         Ok(response) => {
//             let text = response.text().await.unwrap();
//             println!("{}", text);
//         }
//         Err(e) => {
//             eprintln!("Error: {}", e);
//         }
//     }
// }