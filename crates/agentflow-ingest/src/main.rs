use agentflow_ingest::start_ingest;
use agentflow_index::GdpTracker;
use agentflow_classify::AgentHeuristics;
use agentflow_api::start_api;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("🛡️ Geodesic Alpha: AgentFlow Indexer v0.1.0");

    let tracker = Arc::new(GdpTracker::new("./agentflow_db"));
    let heuristics = Arc::new(AgentHeuristics::new());

    // Clone tracker for the API thread
    let api_tracker = tracker.clone();

    // Start API in the background
    tokio::spawn(async move {
        start_api(api_tracker).await;
    });

    // Run the Ingest pipeline in the main thread
    start_ingest(heuristics, tracker).await;
}

