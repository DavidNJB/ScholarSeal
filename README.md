# GreenAnchor

## Project Description
**GreenAnchor** Local Carbon Credit Marketplace: Democratizing climate finance for SMEs and local landowners.

## Problem & Solution
- Problem: Small and medium-sized enterprises (SMEs) often struggle to participate in global carbon offset markets due to high costs, complexity, and lack of accessibility.
- Solution: **GreenAnchor** solves this by enabling local environmental projects (e.g., community reforestation initiatives) to mint tokenized carbon credits as blockchain assets. These credits can then be purchased by local businesses, allowing them to offset their carbon footprint and promote a “carbon-neutral” status—while directly funding local sustainability efforts.

## Timeline
- Phase 1: Foundation: Core Soroban smart contract development, including the project registry and verification logic.
- Phase 2: Validation: Extensive unit testing and local environment simulation to ensure contract security and state persistence.
- Phase 3: Integration: Frontend dashboard development to allow SMEs and project owners to interact with the Stellar network seamlessly.
- Phase 4: Launch: Final deployment to the Stellar Testnet, live verification, and community demo.

## Stellar Features Used
- Soroban Smart Contracts: Manages the registry of verified projects and secure asset transfer logic.
- XLM / USDC Transfers: Facilitates instant, low-cost payments from buyers to environmental projects.
- Custom Tokens (SAC): Represents carbon credits as non-fungible, traceable, and "burnable" assets.
- Trustlines: Ensures secure and authorized holding of project-specific credits

## Project Vision
- To democratize the global carbon market by creating a transparent, low-friction circular economy where local natural capital is recognized, verified, and compensated on-chain.

## Prerequisites
- Rust Toolchain: rustup target add wasm32-unknown-unknown
- Soroban CLI: Version 22.0.0 or higher
- Stellar Laboratory: For account funding on Testnet

## Key Features
- **Decentralized Verification**: A registry system within Soroban to whitelist legitimate environmental projects.
- **On-Chain Minting**: Automated issuance of fractionalized carbon tokens based on verified environmental impact.
- **Secure Offset Purchases**: Peer-to-peer credit transfers between SMEs and projects using Stellar's high-speed settlement.
- **Transparent Retirement**: On-chain logging of "retired" credits to prevent double-counting.
- **Low-Cost Transactions**: Leveraging Stellar’s micro-fees to make small offset purchases viable.

## Deployed Contract Details
- **Network**: Stellar Testnet
- **Contract ID**: `https://stellar.expert/explorer/testnet/tx/d1ecfa552f70bf4ac53c94b496d8d08d752017cceb3018102e36516e8b743a1c`
- **Stellar Expert Link**: `https://lab.stellar.org/r/testnet/contract/CDU7J3446GQBHE5JCSRDE3VPD6FRIVJMEPN2ODBT335YATQ7GGTH2NHN`

## How to Build
- soroban contract build

## How to Test
- cargo test

## How to Deploy to Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/green_anchor.wasm \
  --source <YOUR_ACCOUNT_NAME> \
  --network testnet

  ## Sample CLI Invocation
  - After deployment, you can call the MVP function to purchase an offset with dummy arguments:
  # SME purchases 10 tons of carbon offset from a verified project
soroban contract invoke --id <CONTRACT_ID> \
  --source sme_account \
  --network testnet \
  -- offset_purchase \
  --buyer G_SME_ADDRESS_EXAMPLE \
  --project G_FARMER_ADDRESS_EXAMPLE \
  --amount 10

## Future Scope
- Oracle Integration: Connecting to satellite data to automatically trigger credit minting.
- Local Anchor Integration: Partnering with regional anchors to allow farmers to off-ramp earnings to local bank accounts.
- NFT Impact Certificates: Issuing unique tokens for businesses to display as proof of carbon-neutral status.
