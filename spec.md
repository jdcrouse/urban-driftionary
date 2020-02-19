# Urban Driftionary API Spec

## Get the definition of a term
#### Request
GET `/define/<term>`
### Example Response
If the term has a definition:
```
{
    "term": "Hello",
    "definitions": [
        {
            "definition": "A Greeting",
            "example_sentence": "Hello, world!"
        },
        {
            "definition": "The opposite of goodbye",
            "example_sentence": "Don't say goodbye when you arrive, say hello."
        }
    ]
}
```
Else:
```
{
    "error": "Term is not defined yet"
} 
```

---

## Add a definition for a term
#### Request
POST `/add`

body:
```
{
    "term": "Hello",
    "definition": "This is another definition for 'Hello'",
    "example_sentence": "Hello from the other side."
}
```

---

## Request for someone to define a term for you
#### Request
POST `/request`

body:
```
{
    "term": "Hello"
}
```