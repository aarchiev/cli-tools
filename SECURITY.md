## Security Policy

### Multi-Language CLI Tools Collection
This repository is a monorepo containing various cybersecurity and utility tools built with **Rust**, **JavaScript (Node.js)**, and **Python**. Due to the nature of these tools, we prioritize code security and reliability.

### Supported Versions
Security updates and patches are exclusively provided for the latest code residing in the `master` branch.

| Language | Tools Location | Security Status |
| :--- | :--- | :--- |
| **Rust** | `cli-tools/rust-*` | Supported :white_check_mark: |
| **JavaScript** | `cli-tools/js-*` | Supported :white_check_mark: |
| **Python** | `cli-tools/py-*` | Supported :white_check_mark: |

### Reporting a Vulnerability
If you identify a security flaw (such as memory corruption in Rust, insecure dependencies in Node.js, or logic vulnerabilities in Python), please follow these guidelines:

1.  **Private Disclosure Only:** Do not open a public Issue for security vulnerabilities. This helps protect the community from potential exploits before a fix is deployed.
2.  **Contact:** Please send a detailed report to `[Your Email/Contact]`.
3.  **Submission Details:** Include the specific tool affected, a Proof of Concept (PoC), and the assessed impact.

### Security Practices
We implement language-specific security checks to ensure tool integrity:
* **Rust:** Regular use of `cargo audit` and minimizing `unsafe` code.
* **Python:** Dependency scanning via `pip-audit`.
* **Node.js:** Vulnerability assessment using `npm audit`.

### Apache License 2.0 & Disclaimer
Under the **Apache License 2.0**, all software in this repository is provided "AS IS."

> [!WARNING]
> **Educational and Authorized Use Only.** Any misuse of these tools for illegal activities is strictly the responsibility of the user. The author assumes no liability for damages or legal issues arising from the use of this repository.
