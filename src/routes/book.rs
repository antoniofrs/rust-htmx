use askama::Template;
use axum::response::Html;

use crate::support::renderer::render;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    id: String,
}

#[derive(Template)]
#[template(path = "book_row.html")]
struct BookRow {
    book: Book,
}

#[derive(Template)]
#[template(path = "book_list.html")]
struct BookList {
    list: Vec<Book>,
}

pub async fn get_users() -> Html<String> {
    let books: Vec<Book> = vec![
        Book {
            title: String::from("Il nome della rosa"),
            author: String::from("Umberto Eco"),
            id: String::from("1"),
        },
        Book {
            title: String::from("1984"),
            author: String::from("George Orwell"),
            id: String::from("2"),
        },
        Book {
            title: String::from("Orgoglio e pregiudizio"),
            author: String::from("Jane Austen"),
            id: String::from("3"),
        },
    ];

    let template = BookList { list: books };

    render(template)
}

pub async fn post_book(
    
) -> Html<String> {


    let template = BookRow { list: books };

    render(template)
}
