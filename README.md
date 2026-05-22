# Title
Dojo Tracker - Decentralized Martial Arts Management System

# Description
Traditional martial arts clubs (dojos) rely on paper notebooks or centralized databases to track student attendance, belt promotions, and tuition fees. Dojo Tracker brings this management system to the Stellar network using Soroban smart contracts. It creates an immutable, transparent, and automated record of a practitioner's training journey, ensuring that their hard work and payments are permanently secured on the blockchain.

# Features
- **Admin Initialization:** Securely assign the dojo instructor as the contract administrator.
- **Student Registration:** Enroll new martial arts students starting at the white belt level.
- **On-chain Attendance & Auto-Promotion:** Immutable check-ins for training sessions. The contract automatically upgrades a student's belt level upon reaching specific attendance milestones (e.g., every 24 sessions).
- **Tuition Fee System:** Integrated Soroban token functionalities allowing students to pay tuition fees directly to the master's wallet. The contract permanently updates the `total_fees_paid` state on-chain.

# Contract
Contract link: https://stellar.expert/explorer/testnet/tx/e017a6fdb1a3d7be4b125fc50fc931bf9d64a776450cbc3617d14da64fb016d5

### Contract's screenshots
<img width="1863" height="876" alt="image" src="https://github.com/user-attachments/assets/b7773183-6e69-4d26-94b8-6e4546263ed8" />

<img width="1860" height="879" alt="image" src="https://github.com/user-attachments/assets/f5572241-6021-4ec7-b22f-9850634a0a14" />



# Future scopes
- Implement a web-based UI (using React.js and Freighter API) for the instructor to easily scan student QR codes for instant attendance logging.
- Support multi-currency tuition payments leveraging Stellar's native path payment protocols.

# Profile
**Pham Tan Nhat Thinh - 24127244**
- IT Student at University of Science, VNU-HCMUS (Class 24C06).
- Passionate about Computer Science, AI (Heuristic search algorithms), Operating Systems development, and exploring Decentralized technologies.
