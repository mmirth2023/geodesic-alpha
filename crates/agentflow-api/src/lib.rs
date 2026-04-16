use axum::{routing::get, Json, Router};
use std::sync::Arc;
use agentflow_index::GdpTracker;
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
pub struct GdpResponse {
    total_machine_gdp_lamports: u64,
    estimated_usd_value: f64,
    signal_strength: String,
    status: String,
}

pub async fn start_api(tracker: Arc<GdpTracker>) {
    let app = Router::new().route("/gdp", get(move || {
        let t = tracker.clone();
        async move {
            let total = t.get_total_gdp();
            let sol_price = 85.11; // April 2026 Price
            let usd = (total as f64 / 1_000_000_000.0) * sol_price;

            Json(GdpResponse {
                total_machine_gdp_lamports: total,
                estimated_usd_value: usd,
                signal_strength: if usd > 5000.0 { "HIGH".to_string() } else { "NORMAL".to_string() },
                status: "Geodesic_Active".to_string(),
            })
        }
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

