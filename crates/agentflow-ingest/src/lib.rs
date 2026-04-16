use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_client::rpc_config::RpcBlockConfig;
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use agentflow_classify::AgentHeuristics;
use agentflow_index::GdpTracker;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn start_ingest(heuristics: Arc<AgentHeuristics>, tracker: Arc<GdpTracker>) {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    
    println!("🛡️ Geodesic Alpha: Milestone 5 [Alpha Sentinel Mode]");
    let mut current_slot = client.get_slot().unwrap_or(413574173);

    loop {
        let config = RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::Json),
            max_supported_transaction_version: Some(0),
            ..Default::default()
        };

        match client.get_block_with_config(current_slot, config) {
            Ok(block) => {
                if let Some(transactions) = block.transactions {
                    let mut block_gdp = 0u64;
                    let mut agents = 0;
                    for tx in transactions {
                        if let Some(meta) = tx.meta {
                            let cu = match meta.compute_units_consumed {
                                OptionSerializer::Some(val) => Some(val),
                                _ => None,
                            };
                            if let Some(c) = cu {
                                if heuristics.classify_by_compute(c) {
                                    block_gdp += heuristics.calculate_machine_value(c);
                                    agents += 1;
                                }
                            }
                        }
                    }
                    tracker.add_agent_value("MAINNET", block_gdp);
                    println!("🤖 [SENTINEL] Slot: {} | Agents: {} | GDP: +{} lamports", current_slot, agents, block_gdp);
                }
                current_slot += 1;
            }
            Err(_) => { sleep(Duration::from_millis(450)).await; }
        }
    }
}

