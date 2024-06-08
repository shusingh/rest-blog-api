# Blog API

This is a RESTful API for managing blog posts, built with Rust and Actix-Web. It supports creating, reading, updating, and deleting blog posts.

## Features

- Create a new blog post
- Retrieve a blog post by ID
- Update an existing blog post
- Delete a blog post by ID

## Technologies Used

- Rust
- Actix-Web
- Serde
- UUID
- Chrono

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/)
- Cargo (included with Rust)

### Installation

1. **Clone the repository:**

   ```
   sh
   git clone https://github.com/your-username/blog_api.git
   cd blog_api
   ```
2. **Build and run the application:**
    ```
    cargo run
    ```
The server will start on 127.0.0.1:8080.

### API Endpoints

* **Create a post:**
  ```
  curl -X POST -H "Content-Type: application/json" -d '{"title": "First Post", "content": "This is the content of the first post"}' http://127.0.0.1:8080/posts
  ```
Example Response:
```
{
  "id": "d3b07384-d9b1-11eb-b8bc-0242ac130003",
  "title": "First Post",
  "content": "This is the content of the first post",
  "timestamp": 1625075113
}

```

* **Get a post by ID:**
```
curl http://127.0.0.1:8080/posts/{id}
```
Replace {id} with the actual UUID of the post.


Example response:
```
{
  "id": "d3b07384-d9b1-11eb-b8bc-0242ac130003",
  "title": "First Post",
  "content": "This is the content of the first post",
  "timestamp": 1625075113
}
```

* **Update a post by ID:**

```
curl -X PUT -H "Content-Type: application/json" -d '{"title": "Updated Title", "content": "Updated content"}' http://127.0.0.1:8080/posts/{id}
```
Replace {id} with the actual UUID of the post.

Example response:

```
{
  "id": "d3b07384-d9b1-11eb-b8bc-0242ac130003",
  "title": "Updated Title",
  "content": "Updated content",
  "timestamp": 1625075113
}
```

* **Delete a post by ID:**
```
curl -X DELETE http://127.0.0.1:8080/posts/{id}
```
Replace {id} with the actual UUID of the post.

Example response:

```
"Post d3b07384-d9b1-11eb-b8bc-0242ac130003 deleted"
```

