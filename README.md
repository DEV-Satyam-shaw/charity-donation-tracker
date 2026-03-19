# charity-donation-tracker (Soroban Smart Contract)
A decentralized charity donation tracking system built using Soroban smart contracts on the Stellar network to ensure transparency, accountability, and secure fund management

<img width="1920" height="1080" alt="Screenshot (31)" src="https://github.com/user-attachments/assets/7a5a3b2d-9448-4cba-8d28-31242f2d153d" />
<img width="1920" height="1080" alt="Screenshot (32)" src="https://github.com/user-attachments/assets/d88a5156-976a-47e0-8760-2a65d7352022" />


## 📌 Project Description
The Charity Donation Tracker is a decentralized application built on the Stellar Soroban smart contract platform. It enables transparent and secure tracking of donations made to various charities. The system ensures that all contributions are recorded immutably on-chain, providing trust and accountability.

## 🚀 What it does
- Allows users (donors) to donate tokens to a specific charity.
- Stores total donations received by each charity on-chain.
- Provides a function to query total donations for any charity.

## ✨ Features
- 🔒 Secure donation tracking using blockchain
- 📊 Transparent and immutable record of donations
- 👥 Supports multiple charities
- ⚡ Lightweight and efficient Soroban smart contract
- 🔍 Easy retrieval of total donations per charity

## 🛠 Tech Stack
- Stellar Soroban (Smart Contracts)
- Rust (Contract Language)

## 📦 Functions

### 1. donate(donor, charity, amount)
Allows a user to donate a specified amount to a charity.

### 2. get_total(charity)
Returns the total donations received by a specific charity.

## 🔗 Deployed Smart Contract Link
🔗 https://stellar.expert/explorer/testnet/tx/48eb5ac7652b17ef5fb52999b4e5f92d17051923e34f5b97f878eee7f9602470
🔗 https://lab.stellar.org/r/testnet/contract/CAFV2WGHQM2FM4BSV7XXP7TR27BBVGIZ6XNUCOWZFBUK2LBREW57DPY6

## 📖 How to Use
1. Deploy the contract on Stellar Soroban.
2. Call `donate()` with:
   - your address
   - charity name (symbol)
   - amount
3. Call `get_total()` to view total funds for a charity.

## 📌 Future Improvements
- Track individual donor contributions
- Add withdrawal functionality for charities
- Add event logging for donations
- Integrate frontend dashboard
