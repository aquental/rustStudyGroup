# build the JSON payload
$payload = @{
    course_type = "online"
    title       = 'Rust Fundamentals'
    instructor  = 'Jane Developer'
    description = 'A hands‑on introduction to Rust'
    price       = 49.99
    url         = 'https://academy.example.com/rust‑fundamentals'
} | ConvertTo-Json

# POST to /courses/online
Invoke-RestMethod `
    -Method Post `
    -Uri 'http://localhost:3000/courses' `
    -ContentType 'application/json' `
    -Body $payload

# 2

Invoke-RestMethod -Uri "http://localhost:3000/courses" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{
    "course_type": "online",
    "title": "Rust for Beginners",
    "instructor": "Alice",
    "description": "A complete course on Rust",
    "price": 49.99,
    "url": "https://example.com/rust"
  }'