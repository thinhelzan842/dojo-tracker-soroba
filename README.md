# Title
Dojo Tracker - Decentralized Martial Arts Management System

# Description
Traditional martial arts clubs (dojos) rely on paper notebooks or centralized databases to track student attendance, belt promotions, and tuition fees. Dojo Tracker brings this management system to the Stellar network using Soroban smart contracts. It creates an immutable, transparent, and automated record of a practitioner's training journey, ensuring that their hard work and payments are permanently secured on the blockchain.

## Project Vision
Our vision is to completely eliminate manual paperwork in local martial arts clubs globally. By leveraging Stellar and Soroban, we aim to build a fully decentralized, trustless ecosystem where student progression, belt ranks, and tuition payments are automated, transparent, and immutable.

# Features
- **Admin Initialization:** Securely assign the dojo instructor as the contract administrator.
- **Student Registration:** Enroll new martial arts students starting at the white belt level.
- **On-chain Attendance & Auto-Promotion:** Immutable check-ins for training sessions. The contract automatically upgrades a student's belt level upon reaching specific attendance milestones (e.g., every 24 sessions).
- **Tuition Fee System:** Integrated Soroban token functionalities allowing students to pay tuition fees directly to the master's wallet. The contract permanently updates the `total_fees_paid` state on-chain.

## Deployed Contract Details
- **Contract ID:** CAKU6O2ZGVPP7CYJOEPVOD56YQ2Q3YJPOBRIVDIW5KDCRLDUWTN6QB43
- **Network:** Stellar Testnet
- **Contract link**: https://stellar.expert/explorer/testnet/tx/20099ef8bb0ef9323b9c9f4e037e7923ce0e76f6dd06b3b06955b076a4326afe
- 
### Contract's screenshots
![screenshot](https://github.com/user-attachments/assets/11833bc4-0444-42cf-b77f-7b78c3ec6ac1)

![screenshot](https://github.com/user-attachments/assets/b0001291-38f9-4253-af92-a046ffa769a9)

# Future scopes
- Implement a web-based UI (using React.js and Freighter API) for the instructor to easily scan student QR codes for instant attendance logging.
- Support multi-currency tuition payments leveraging Stellar's native path payment protocols.

# Profile
**Pham Tan Nhat Thinh - 24127244**
- IT Student at University of Science, VNU-HCMUS (Class 24C06).
- Passionate about Computer Science, AI (Heuristic search algorithms), Operating Systems development, and exploring Decentralized technologies.
