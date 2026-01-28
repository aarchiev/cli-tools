# Security Policy

## Multi-Language CLI Tools Collection
This repository serves as a monorepo containing various security and utility tools developed in **Rust**, **JavaScript (Node.js)**, and **Python**. Given the nature of cybersecurity tools, we take code integrity and security seriously.

## Supported Versions
We provide security updates and bug fixes only for the latest versions available in the `main` branch.

| Language | Tools Location | Security Status |
| :--- | :--- | :--- |
| **Rust** | `cli-tools/rust-*` | Supported :white_check_mark: |
| **JavaScript** | `cli-tools/js-*` | Supported :white_check_mark: |
| **Python** | `cli-tools/py-*` | Supported :white_check_mark: |

## Reporting a Vulnerability
If you discover a security vulnerability (e.g., memory safety issues in Rust, script injection in Python, or dependency vulnerabilities in JS), please follow these steps:

1.  **Do Not Open a Public Issue:** To prevent exploitation before a fix is available, please avoid reporting vulnerabilities through the public GitHub Issues tab.
2.  **Contact:** Send a detailed report to `[email.unsterile966@passinbox.com]` or via private GitHub security advisories if enabled.
3.  **Report Contents:** Include the specific tool name, steps to reproduce (PoC), and the potential impact.

## Security Standards
We strive to maintain high security standards across all languages used:
* **Rust:** Minimizing `unsafe` blocks and utilizing `cargo audit` for dependency checks.
* **Python:** Auditing packages using `safety` or `pip-audit`.
* **JavaScript:** Running `npm audit` to mitigate supply chain risks.

## Apache License 2.0 & Disclaimer
As per the **Apache License 2.0**, all tools are provided "AS IS" without any warranties.

> [!WARNING]
> **Misuse for illegal activities is strictly prohibited.** The author(s) shall not be held responsible for any damage or legal consequences caused by the use of these tools. Use them exclusively for educational purposes, research, and authorized penetration testing.
