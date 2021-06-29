use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Result,
};

#[get("/tailwind.css")]
async fn tailwind() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/css/tailwind.css")?)
}

#[get("/robots.txt")]
async fn robots() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=UTF-8")
        .header("server", "RegenWeb")
        .body(r"
            User-agent: *
            Disallow:
         "))

}

#[get("/index")]
async fn index(session: Session, req: HttpRequest) -> Result<HttpResponse>  {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .header("server", "RegenWeb")
        .body(include_str!("../../static/index.html")))
}

/// 404 handler
async fn p404() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .header("server", "Cooler Server jaja")
        .body(include_str!("../../static/404.html")))

}


#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    println!("Starting WebServer!");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(tailwind)
            .service(robots)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "index")
                    .finish()
            })))
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })

        .bind("0.0.0.0:8000")?
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
