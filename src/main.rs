use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:7878...");
    server
        .bind("127.0.0.1:7878")
        .expect("Error binding server to address")
        .run()
        .expect("Error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="a"/>
        <input type="text" name="b"/>
        <button type="submit">Compute GCD</button>
        </form>"#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    println!("got here with {} and {}", form.a, form.b);
    if form.a == 0 && form.b == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("GCD(0, 0) is undefined.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.a,
        form.b,
        gcd(form.a, form.b)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

#[derive(Deserialize)]
struct GcdParameters {
    a: u64,
    b: u64,
}

fn gcd(a: u64, b: u64) -> u64 {
    assert!(a != 0 || b != 0);
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
