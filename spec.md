# Urban Driftionary API Spec

## Get the definition of a term
#### GET `/define/<term>`

#### Response:
```
{
    "term": str,
    "definitions": [
        {
            "definition": str,
            "example_sentence": str,
            "tags": [str, ...]
        },
        ...
    ]
}
```

#### Example Response:

If the term has a definition:
```
{
    "term": "Hello",
    "definitions": [
        {
            "definition": "A Greeting",
            "example_sentence": "Hello, world!",
            "tags": ["greeting", "marketing", "sales", "automation"]
        },
        {
            "definition": "The opposite of goodbye",
            "example_sentence": "Don't say goodbye when you arrive, say hello.",
            "tags": ["greeting", "sales", "automation", "grammar"]
        }
    ]
}
```
Else:
```
{
        "status": "error",
        "reason": "Term is not defined."
}
```

---

## Add a definition for a term
#### POST `/add`
```
{
    "term": str,
    "definition": str,
    "example_sentence": str,
    "tags": [str, ...]
}
```

Example Request:
```
{
    "term": "Hello",
    "definition": "This is another definition for 'Hello'",
    "example_sentence": "Hello from the other side.",
    "tags": ["greetings", "marketing", "automation"]
}
```

---

## Request for a term to be defined 
#### POST `/request`
```
{
    "term": str
}
```

#### Example Request:
```
{
    "term": "Hello"
}
```