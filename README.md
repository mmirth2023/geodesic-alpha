# 🛡️ Geodesic Alpha: The Machine GDP Sentinel

**Geodesic Alpha** is a high-performance, modular indexing engine built in Rust, designed to quantify the **Machine-to-Machine (M2M) economy** on Solana. 

As the network transitions into the "Agentic Internet" era (2026), traditional metrics like raw Transaction Count or TVL have become lagging indicators. Geodesic Alpha introduces **Machine GDP (mGDP)**—a deterministic, compute-weighted index that isolates autonomous agent activity from human noise.

---

## 🚀 The Vision: Beyond Vanity Metrics
In a world of automated liquidity, the true health of a network is found in its **Execution Intensity**. Geodesic Alpha analyzes the "Execution Physics" of the Solana Sealevel runtime to identify high-utility agents.

* **Compute-Weighted Analysis:** We filter for transactions consuming **>50k Compute Units (CU)**, isolating sophisticated AI agents, HFT arbitrageurs, and cross-chain aggregators.
* **Machine Certainty Score (MCS):** A proprietary heuristic that determines the probability of a transaction being non-human based on deterministic execution patterns.
* **Zero-Latency Indexing:** Optimized for the **Alpenglow (150ms)** era, providing real-time economic signals before they hit historical data aggregators.

---

## 🛠️ Modular Architecture
The system is built as a highly decoupled Rust workspace, ensuring each component can scale independently:

* **`agentflow-ingest`**: High-throughput RPC/Geyser crawler with pattern-matched metadata extraction.
* **`agentflow-classify`**: The intelligence layer. Applies heuristic signatures to distinguish "Machine Work" from "Sybil Noise."
* **`agentflow-index`**: High-performance persistence layer using **Sled**, an embedded KV store designed for zero-admin high-concurrency workloads.
* **`agentflow-api`**: An Axum-powered REST gateway providing USD-denominated economic signals and real-time mGDP data.

---

## 📱 Edge-Native Efficiency (The PoC)
Uniquely, Geodesic Alpha was architected and stress-tested on **ARM64 architecture via Termux**. 

* **The Challenge:** Indexing Solana Mainnet metadata on mobile hardware with limited resources.
* **The Result:** Successful real-time tracking of 400+ agents per block with sub-500ms latency.
* **The Significance:** By achieving enterprise-level indexing on an edge device, we have proven that the Geodesic Alpha codebase is one of the most resource-efficient engines in the ecosystem. On dedicated NVMe infrastructure, this system is built to lead the market.

---

## 🔮 Future Roadmap

### **Phase 1: Institutional API Access**
Development of a low-latency WebSocket feed designed for institutional quant funds. This provides real-time "Machine Alpha" alerts, allowing funds to track the movement of autonomous capital.

### **Phase 2: Cross-Chain Liquidity Tracking**
Expansion of heuristics to track cross-chain agentic flows (SVM to EVM), identifying where autonomous agents are migrating and why.

### **Phase 3: Integration with Solana Agent Registry**
Working toward becoming the primary verification layer for the official Solana Agent Registry, providing a "Credit Score" for autonomous agents.

---

## 📄 License
This project is licensed under the **Apache License 2.0**.
