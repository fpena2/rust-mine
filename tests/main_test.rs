use actix_web::{http::StatusCode, test, web, App};
use rust_mine::{do_work, WorkResponse, Worker, World};
use std::sync::Mutex;

#[actix_rt::test]
async fn test_do_work() {
    // Create a test App with shared data
    let app_data = web::Data::new(World {
        resources: Mutex::new(10),
    });

    // Create the App and add the routes
    let app = test::init_service(
        App::new()
            .app_data(app_data.clone())
            // Use .route to define the route and specify the handler function
            .route("/do_work", web::post().to(do_work)),
    )
    .await;

    // Create a test HTTP request with the worker data
    let worker = Worker {
        uid: "test_worker".to_string(),
        work: 5,
        worker_credit: 0,
    };

    // Create a test request and execute it
    let req = test::TestRequest::post()
        .uri("/do_work")
        .set_json(&worker)
        .app_data(app_data.clone())
        .to_request();

    // Call application
    let resp = test::call_service(&app, req).await;

    // Check the response status
    assert_eq!(resp.status(), StatusCode::OK);

    // Parse the response body
    let body_bytes = test::read_body(resp).await;
    let work_response: WorkResponse = serde_json::from_slice(&body_bytes).unwrap();

    // Perform assertions on the response
    assert_eq!(work_response.mine_resources, 5);
    assert_eq!(work_response.work_credit, 0);
}
