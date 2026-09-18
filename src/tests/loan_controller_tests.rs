use actix_web::{test, web, App, http::StatusCode};
use serde_json::json;

mod controller_integration_tests {
    use super::*;

    #[actix_web::test]
    async fn test_create_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "2025-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_create_loan_invalid_amount() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": -1000.0,
            "interest_rate": 5.5,
            "due_date": "2025-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_create_loan_invalid_interest_rate() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": -2.0,
            "due_date": "2025-12-31"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_create_loan_past_due_date() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0,
            "interest_rate": 5.5,
            "due_date": "2020-01-01"
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_all_loans_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_loan_by_id_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 15000.0,
            "interest_rate": 4.0,
            "due_date": "2025-06-30"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let create_resp = test::call_service(&app, create_req).await;
        assert_eq!(create_resp.status(), StatusCode::CREATED);

        let req = test::TestRequest::get()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_loan_by_id_not_found() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/loans/99999")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_update_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 10000.0,
            "interest_rate": 3.5,
            "due_date": "2025-09-30"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let update_payload = json!({
            "amount": 12000.0,
            "interest_rate": 4.5
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/1")
            .set_payload(update_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_update_loan_not_found() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 20000.0,
            "interest_rate": 6.0
        });

        let req = test::TestRequest::put()
            .uri("/api/loans/88888")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_delete_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 5000.0,
            "interest_rate": 2.0,
            "due_date": "2025-03-31"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/1")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_delete_loan_not_found() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::delete()
            .uri("/api/loans/77777")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_approve_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 8000.0,
            "interest_rate": 3.0,
            "due_date": "2025-08-31"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let req = test::TestRequest::put()
            .uri("/api/loans/1/approve")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_reject_loan_success() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let create_payload = json!({
            "amount": 25000.0,
            "interest_rate": 7.5,
            "due_date": "2025-11-30"
        });

        let create_req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(create_payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let _create_resp = test::call_service(&app, create_req).await;

        let req = test::TestRequest::put()
            .uri("/api/loans/1/reject")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_missing_required_fields() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let payload = json!({
            "amount": 10000.0
        });

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload(payload.to_string())
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_invalid_json_format() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api").configure(crate::controllers::loan_controller::configure))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/loans")
            .set_payload("not valid json")
            .insert_header(actix_web::http::header::ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}