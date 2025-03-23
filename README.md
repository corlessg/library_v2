This library application can be used to store books into a database.

The application used a MongoDB NoSQL database.

This application can run as a stand-alone web server or as a CLI application.

Both operating modes enable a user to perform normal CRUD operations on the library database, either through CLI commands or typical HTTP requests.

The CLI commands include:

# 📚 Library CLI Commands

This command-line interface (CLI) provides commands to manage a library database. You can search, add, update, and remove books, as well as scan books or batch upload from a CSV file.

---

## 🛠️ How to Use

Run the CLI with one of the subcommands below.

```bash
cargo run --bin cli <command> [options]
```

---

## Commands

### 🔎 `search`

Search for books in the library.

#### Subcommands:

#### `isbn`
Search for a book by its ISBN.

```bash
cargo run --bin cli search isbn <isbn>
```

- **Arguments**:
  - `<isbn>` (required): The ISBN number of the book.
  
**Example**:
```bash
cargo run --bin cli search isbn 9783161484100
```

---

#### `title`
Search for books by their title.

```bash
cargo run --bin cli search title <title>
```

- **Arguments**:
  - `<title>` (required): The title of the book.

**Example**:
```bash
cargo run --bin cli search title "The Rust Programming Language"
```

---

### ➕ `add`

Add a new book to the library.

```bash
cargo run --bin cli add <isbn>
```

- **Arguments**:
  - `<isbn>` (required): The ISBN number of the book to add.

**Example**:
```bash
cargo run --bin cli add 9783161484100
```

---

### ❌ `remove`

Remove a book from the library by its ISBN.

```bash
cargo run --bin cli remove <isbn>
```

- **Arguments**:
  - `<isbn>` (required): The ISBN number of the book to remove.

**Example**:
```bash
cargo run --bin cli remove 9783161484100
```

---

### ✏️ `update`

Update the location information of a book by its ISBN.

```bash
cargo run --bin cli update <isbn> [house] [room] [owner]
```

- **Arguments**:
  - `<isbn>` (required): The ISBN number of the book to update.
  - `[house]` (optional): The house where the book is stored.
  - `[room]` (optional): The room within the house.
  - `[owner]` (optional): The owner of the book.

**Example**:
```bash
cargo run --bin cli update 9783161484100 --house "Main Library" --room "2A" --owner "Alice"
```

---

### 📷 `scanner`

Start scanner mode to enter ISBN numbers rapidly to perform operations on the database.

```bash
cargo run --bin cli scanner
```

- Runs a continuous program where a user can scan ISBN numbers continuously for rapid book entry, removal, checkin, and checkout processes. The ideal use-case is to use a barcode scanner plugged into the computer running this program to continuously perform library functions.

---

### 📂 `batch`

Batch upload books from a CSV file.

```bash
cargo run --bin cli batch <file_path>
```

- **Arguments**:
  - `<file_path>` (required): Path to the CSV file containing book data.

**Example**:
```bash
cargo run --bin cli batch ./books.csv
```

---

## ✅ Command Overview

| Command                       | Description                                            |
|-------------------------------|--------------------------------------------------------|
| `search isbn`                 | Search for a book by its ISBN                         |
| `search title`                | Search for books by their title                      |
| `add`                         | Add a book to the library database                   |
| `remove`                      | Remove a book from the library by ISBN               |
| `update`                      | Update book location or owner information            |
| `scanner`                     | Start scanner mode for rapid ISBN entry              |
| `batch`                       | Batch upload books from a CSV file                   |

---

## ⚙️ Notes

- `isbn` is always required for operations related to a specific book.
- Optional fields in `update` allow partial updates (house, room, owner).
- The `scanner` mode is useful when working with barcode scanners.
- The CSV file for `batch` upload should be formatted correctly (see CSV specs for details).

---

Web-Server Routes:

### 📖 `GET /books/<id>`

Retrieve a book by its ISBN.

- **URL:** `/books/<id>`
- **Method:** `GET`
- **Path Params:**
  - `id` (string): The ISBN of the book you want to retrieve.
- **Success Response:**
  - `200 OK`
  - Returns a JSON object representing the book.
- **Error Response:**
  - `500 Internal Server Error` (Custom error message)
  
**Example Request:**
GET /books/9783161484100

---

### 📚 `GET /books`

Retrieve a list of random books from the library.

- **URL:** `/books`
- **Method:** `GET`
- **Success Response:**
  - `200 OK`
  - Returns a JSON array of random books.
- **Error Response:**
  - `500 Internal Server Error` (Custom error message)
  
**Example Request:**
GET /books

---

### ➕ `POST /books`

Create a new book in the library.

- **URL:** `/books`
- **Method:** `POST`
- **Request Body:**
  - Raw string (text/plain) representing the ISBN of the book.
- **Success Response:**
  - `201 Created`
  - Returns a JSON object of the newly created book.
- **Error Response:**
  - `500 Internal Server Error` (Custom error message)

**Example Request:**
POST /books Body: 9783161484100

---

### ✏️ `PUT /books/<id>`

Update the location of an existing book by its ISBN.

- **URL:** `/books/<id>`
- **Method:** `PUT`
- **Path Params:**
  - `id` (string): The ISBN of the book to update.
- **Request Body (JSON):**
  ```json
  {
    "house": "arlington",
    "room": "library",
    "owner":"John Smith"
  }

 ---

### ❌ `DELETE /books`

Delete a book from the library by its ISBN.

- **URL:** `/books`
- **Method:** `DELETE`
- **Request Body:**
  - Raw string (text/plain) representing the ISBN of the book to delete.
- **Success Response:**
  - `201 Created` (Indicates deletion succeeded)
  - Returns a JSON object of the deleted book.
- **Error Response:**
  - `500 Internal Server Error` (Custom error message)

**Example Request:**

DELETE /books HTTP/1.1 Host: <host>:<port> Content-Type: text/plain

9783161484100

