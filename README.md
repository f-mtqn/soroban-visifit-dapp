# 🏋️‍♂️ VisiFit Protocol: Proof-of-Kinetic Fitness dApp

![Stellar](https://img.shields.io/badge/Network-Stellar_Testnet-black?style=for-the-badge&logo=stellar)
![Soroban](https://img.shields.io/badge/Smart_Contract-Soroban-orange?style=for-the-badge)
![Rust](https://img.shields.io/badge/Language-Rust-red?style=for-the-badge&logo=rust)

## 🌟 The Vision: Fixing "Move-to-Earn"

The current generation of "Move-to-Earn" (M2E) applications is fundamentally flawed. Because they rely entirely on mobile GPS or basic accelerometers, they are incredibly easy to exploit—users can simply shake their phones or attach them to automated devices to farm tokens. 

**VisiFit** introduces a paradigm shift: **Proof-of-Kinetic**. 

Instead of trusting device sensors, VisiFit uses a Web2.5 hybrid architecture. We utilize off-chain Computer Vision (via webcam) to track human body landmarks and mathematically verify perfect exercise forms (e.g., ensuring a squat hits the correct depth). Once the physical effort is verified, an automated orchestrator acts as a secure Oracle, triggering this Soroban smart contract to permanently record the exact number of verified reps on the Stellar blockchain.

**Your body is the miner, and your sweat is the cryptographic proof.**

---

## 🏗️ System Architecture

To keep transaction costs low and bypass the computational limits of blockchains, VisiFit distributes the workload:

1. **The Frontend (Off-Chain AI):** A web application utilizing browser-based Computer Vision (like MediaPipe) to track 33 3D body landmarks in real-time. It counts reps only when the correct kinetic form is achieved.
2. **The Oracle (Off-Chain Orchestrator):** A secure middleware automation server (e.g., n8n). It receives the encrypted workout payload, verifies the timestamp and liveness data to prevent replay attacks, and acts as the authorized "Admin."
3. **The Ledger (On-Chain Smart Contract):** This repository. A Soroban Smart Contract written in Rust. It securely stores the state (total reps) for each user. The contract is strictly permissioned so that *only* the authorized AI Oracle can append new verified workouts.

---

## 📂 Project Structure

This repository follows the recommended Soroban workspace structure:

```text
.
├── contracts
│   └── visifit
│       ├── src
│       │   ├── lib.rs     # Core Smart Contract Logic (Init, Record, Fetch)
│       │   └── test.rs    # Unit tests simulating Oracle authorization & state changes
│       └── Cargo.toml     # Contract-specific dependencies
├── Cargo.toml             # Workspace dependency manager
└── README.md              # Project documentation
```

## 🌐 Live Deployment
This smart contract has been successfully deployed to the Stellar Testnet.
* **Network:** Stellar Testnet
* **Contract ID:** `CDM4LUNEYEZMKJOYFXHFLYF6HFXK2AUMO6H6MA4HDX2WW7Z6WC7P6B76`
* **Explorer Link:** [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CDM4LUNEYEZMKJOYFXHFLYF6HFXK2AUMO6H6MA4HDX2WW7Z6WC7P6B76)
