use actix_web::test;
use actix_web::App;

#[actix_web::test]
async fn test_connect() {
    let app = test::init_service(
        App::new().service(actix_web::web::resource("/").to(|| async { "Hello, client!" })),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), 200);
    assert_eq!(test::read_body(res).await, "Hello, client!");
}
