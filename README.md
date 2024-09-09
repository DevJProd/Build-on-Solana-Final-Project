# Web3 Restaurant Review DApp - README

Welcome to the **Restaurant Review dApp** project repository! This decentralized application (dApp) leverages Solana blockchain technology to create a secure and transparent platform where users can submit reviews for restaurants. Users can provide feedback by submitting a title, description, location, and rating for the restaurant, all of which is stored immutably on the blockchain.

Participants can interact with the dApp to leave reviews and browse feedback from others, ensuring the integrity of reviews through smart contract-based validation and storage.

![image](https://github.com/DevJProd/Restaurant-Review-project-Final-BOS/blob/main/assets/frontend.png)

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Smart Contract Deployment](#smart-contract-deployment)
  - [Deployment Steps](#deployment-steps)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Smart Contracts](#smart-contracts)
- [Testing through Frontend Client](#testing-through-frontend-client)
- [Frontend](#frontend)
- [Contributing](#contributing)
- [License](#license)

## Overview

The **Restaurant Review dApp** allows users to submit reviews for various restaurants through a decentralized platform powered by Solana smart contracts. Users can add reviews by providing a title, description, location, and rating, ensuring that all feedback is securely stored and transparent on the blockchain. The use of smart contracts guarantees integrity and prevents tampering with reviews, fostering a trusted review system.

## Features

- Submit Restaurant Reviews: Users can add reviews with a title, description, location, and rating (1-10) directly  on the blockchain.
- Browse Reviews: View reviews submitted by others for different restaurants.
- Decentralized Storage: Reviews are stored on the blockchain, ensuring they cannot be altered or deleted.
- Real-time Updates: Instantly see new reviews as they are added.
- Solana Wallet Integration: Connect your Solana wallet (e.g., Phantom) to securely interact with the dApp.
- Trust & Transparency: Each review is backed by a smart contract, ensuring authenticity.

## Getting Started

Follow these steps to deploy the smart contract and start submitting and browsing restaurant reviews on the blockchain through your frontEnd dApp.

## Smart Contract Deployment

The smart contracts (folder composed of the 3 rust files) for this project are expected to be deployed on the Solana devnet using Solana PlayGround. Solana PlayGround is an intuitive web IDE provided by Solana, designed to simplify the development and deployment of smart contracts.

![image](https://github.com/DevJProd/Restaurant-Review-project-Final-BOS/blob/main/assets/playGround.png)

## Deployment Steps

1. Create a New Project:

Go to the Solana PlayGround interface and create a new project, select Rust as the framework.

2. Add Rust Files:

Add the provided Rust files under program/src folder to the project.

3. Build the Smart Contract:

Navigate to the "Build and Deploy" section in the left vertical bar. Click the "Build" button to ensure the smart contract is built without errors. Wait for the "Build successful" message to confirm that the build process has completed successfully.

4. Deploy the Smart Contract:

Once the build is successful, click the "Deploy" button. Wait for the "Deployment successful" message to confirm that the smart contract has been deployed successfully.

### Prerequisites

1. Node.js: Ensure Node.js is installed. Download it from [nodejs.org](https://nodejs.org/).

### Installation

1. Clone the repository:

```bash
  git clone https://github.com/DevJProd/Restaurant-Review-project-Final-BOS.git
```

2. Navigate to the project directory:

```bash
  cd Restaurant-Review-project-Final-BOS
```

3. Install required npm packages:

```bash
 npm install
```

## Usage

1. Start the development server:

```bash
 npm run dev
```

2. Open your web browser and navigate to `http://localhost:3000` to access the DApp.

3. Connect your Solana wallet (e.g., Phantom, Solflare) to the DApp.

4. Check submitted reviews, add reviews, and update previous reviews.

## Smart Contracts

The smart contracts in this project facilitates the review process for restaurants. It manages the addition and updating of reviews, including details like the review's title, description, location, and rating. These contracts are deployed on the Solana blockchain, ensuring secure and transparent interactions between users and the system..

- `instruction.rs`: This file defines how the smart contract will handle and decode instructions related to adding and updating restaurant reviews. The reviews are composed of a title, rating, description, and location, and this struct is designed to properly parse and handle these inputs when interacting with the Solana blockchain.

- `lib.rs`: This smart contract provides two key functionalities,Adding Reviews where Users can submit a new review, which is stored in a PDA (derived from their public key and the review title) and Updating Reviews where Users can update an existing review, but only if they originally created it. The system ensures security by validating the PDA for each review, and it uses Solana's system program to create accounts and manage rent..

- `state.rs`: Defines the data structure used to store review details and the custom error handling for the smart contract.

## Testing through Frontend Client

A user-friendly dApp frontend will be provided to seamlessly interact with and test the functionality of the restaurant review smart contract, allowing you to effortlessly add and update reviews directly from your browser.

The dApp frontend is built using modern web technologies including React.js. It provides an intuitive and interactive user interface for review submitting.

- **React.js**: Powers the DApp's user interface.
- **solana/web3.js**: The Solana JavaScript API for smart contract interaction.
- **@solana/wallet-adapter-react-ui**: Solana wallet browser extension for secure transactions.

## Contributing

Contributions to this project are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature/bug fix.
3. Make changes and test thoroughly.
4. Commit with clear and concise messages.
5. Push changes to your fork.
6. Submit a pull request describing your changes.

## License

This project is licensed under the [MIT License](LICENSE).

---

Thank you for your interest in the Web3 Restaurant Review DApp project! For questions or suggestions, reach out to us or open an issue on [GitHub](https://github.com/Rise-In/XXX-Bootcamp-FinalCase). Happy bidding on the blockchain! 🚀
