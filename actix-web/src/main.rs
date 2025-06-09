use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

#[get("/hello-world")]
async fn hello_world() -> impl Responder {
    "Hello World!"
}

#[get("/temperature")]
async fn current_temperature() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello_world)
            .service(current_temperature)
            .service(hello)  // This catch-all route must be last
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_index_handler() {
        let app = test::init_service(
            App::new().service(index)
        ).await;

        let req = test::TestRequest::get()
            .uri("/")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, World!");
    }

    #[actix_web::test]
    async fn test_hello_handler() {
        let app = test::init_service(
            App::new().service(hello)
        ).await;

        let req = test::TestRequest::get()
            .uri("/John")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello John!");
    }

    #[actix_web::test]
    async fn test_hello_handler_with_special_characters() {
        let app = test::init_service(
            App::new().service(hello)
        ).await;

        let req = test::TestRequest::get()
            .uri("/Alice%20Bob")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello Alice Bob!");
    }

    #[actix_web::test]
    async fn test_full_app_integration() {
        let app = test::init_service(
            App::new().service(index).service(hello)
        ).await;

        // Test index route
        let req = test::TestRequest::get()
            .uri("/")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        // Test hello route
        let req = test::TestRequest::get()
            .uri("/World")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello World!");
    }

    #[actix_web::test]
    async fn test_catch_all_route() {
        let app = test::init_service(
            App::new().service(index).service(hello)
        ).await;

        // Test that the /{name} route catches any single path segment
        let req = test::TestRequest::get()
            .uri("/unknown")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello unknown!");
    }

    #[actix_web::test]
    async fn test_multi_segment_path_404() {
        let app = test::init_service(
            App::new().service(index).service(hello)
        ).await;

        // Multi-segment paths should return 404
        let req = test::TestRequest::get()
            .uri("/path/to/nowhere")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_method_post_not_allowed() {
        let app = test::init_service(
            App::new().service(index).service(hello)
        ).await;

        let req = test::TestRequest::post()
            .uri("/")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);  // POST method not specifically handled
    }

    #[actix_web::test]
    async fn test_hello_world_handler() {
        let app = test::init_service(
            App::new().service(hello_world)
        ).await;

        let req = test::TestRequest::get()
            .uri("/hello-world")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello World!");
    }

    #[actix_web::test]
    async fn test_current_temperature_handler() {
        let app = test::init_service(
            App::new().service(current_temperature)
        ).await;

        let req = test::TestRequest::get()
            .uri("/temperature")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        // Check content type is JSON
        let content_type = resp.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("application/json"));
        
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert_eq!(body_str, "{\"temperature\":42.3}");
    }

    #[actix_web::test]
    async fn test_temperature_response_structure() {
        let app = test::init_service(
            App::new().service(current_temperature)
        ).await;

        let req = test::TestRequest::get()
            .uri("/temperature")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        
        // Parse JSON to ensure it's valid
        let parsed: serde_json::Value = serde_json::from_str(body_str).unwrap();
        assert!(parsed.get("temperature").is_some());
        assert_eq!(parsed["temperature"], 42.3);
    }

    #[actix_web::test]
    async fn test_complete_app_with_all_routes() {
        let app = test::init_service(
            App::new()
                .service(index)
                .service(hello_world)
                .service(current_temperature)
                .service(hello)  // Catch-all route must be last
        ).await;

        // Test all routes
        let routes = vec![
            ("/", "Hello, World!"),
            ("/hello-world", "Hello World!"),
            ("/TestUser", "Hello TestUser!"),
        ];

        for (path, expected_body) in routes {
            let req = test::TestRequest::get()
                .uri(path)
                .to_request();
            
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success(), "Route {} failed", path);
            
            let body = test::read_body(resp).await;
            assert_eq!(body, expected_body, "Route {} returned wrong body", path);
        }

        // Test temperature route separately due to JSON response
        let req = test::TestRequest::get()
            .uri("/temperature")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert_eq!(body_str, "{\"temperature\":42.3}");
    }
}
