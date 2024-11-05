#[derive(Debug, Clone)]
pub struct Book {
    author: String,
    name: String,
}

pub struct Library {
    books: Vec<Book>,
}

pub trait LibraryManager {
    fn new() -> Self;
    fn add_book(&mut self, book: Book);
    fn get_book(&self, index: usize) -> Option<&Book>;
    fn update_book(&mut self, index: usize, book: Book) -> bool;
    fn delete_book(&mut self, index: usize) -> bool;
    fn list_books(&self);
}

impl LibraryManager for Library {
    // Metodo per inizializzare la libreria (crea un vettore vuoto)
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    // Aggiunge un nuovo libro alla libreria
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // Recupera un libro in base all'indice
    fn get_book(&self, index: usize) -> Option<&Book> {
        self.books.get(index)
    }

    // Aggiorna i dettagli di un libro in base all'indice
    fn update_book(&mut self, index: usize, book: Book) -> bool {
        if let Some(existing_book) = self.books.get_mut(index) {
            *existing_book = book;
            true
        } else {
            false
        }
    }

    // Elimina un libro dalla libreria in base all'indice
    fn delete_book(&mut self, index: usize) -> bool {
        if index < self.books.len() {
            self.books.remove(index);
            true
        } else {
            false
        }
    }

    // Elenca tutti i libri nella libreria
    fn list_books(&self) {
        for (i, book) in self.books.iter().enumerate() {
            println!("{}: {} by {}", i, book.name, book.author);
        }
    }
}