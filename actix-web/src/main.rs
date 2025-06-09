use actix_web::{get, web, App, HttpServer, Responder};

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
    HttpServer::new(|| App::new().service(index).service(hello))
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
}
