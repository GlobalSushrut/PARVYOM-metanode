#[test]
fn bpi_core_08_http_cage_demo_app_template_served() {
    println!("=== Test: BPI-CORE-08: HTTP Cage demo app template served ===");

    // Load the same demo app HTML that VmServer uses (compile-time include)
    let html = include_str!("../web_apps/httpcg_demo_app.html");

    println!("demo_app_html_len: {} bytes", html.len());

    let preview_len = html.len().min(200);
    let preview = &html[..preview_len];
    println!("demo_app_html_preview (first {} chars):\n{}\n--- end preview ---", preview_len, preview);

    // Basic shape checks
    assert!(html.to_lowercase().contains("<html>"));
    assert!(html.to_lowercase().contains("</html>"));

    // Core demo placeholder signature from test plan
    assert!(
        html.contains("<h1>HTTP CG Demo Placeholder</h1>"),
        "demo app HTML must contain HTTP CG Demo placeholder heading",
    );

    assert!(
        html.contains("<title>HTTP CG Demo</title>"),
        "demo app HTML must contain HTTP CG Demo title",
    );

    println!("status: OK");
}
