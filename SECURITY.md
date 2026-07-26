# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in EZHTML, please report it
privately by emailing **ezhtml@flazzy.de**.

Please DO NOT open a public GitHub issue for security vulnerabilities.

We will respond within 48 hours and aim to publish a fix as soon as possible.

## Scope

The following are in scope for security reports:

- Compiler bugs that can lead to arbitrary code execution when parsing
  untrusted `.ezhtml` files.
- Editor sandbox-escape issues.
- VS Code extension command injection.
- Live preview XSS vectors that bypass the sandbox.
- Supply-chain risks in our release pipeline.

## Out of Scope

- Denial-of-service attacks against the website.
- Issues in third-party dependencies that do not directly affect EZHTML.

## Secrets

Never commit API keys, tokens, passwords or other secrets to this repository.
The `.gitignore` file at the root already blocks common patterns. If you
accidentally commit a secret, treat it as compromised: rotate it immediately
and notify the maintainers.

## After the disclosure

Once a vulnerability is fixed and an advisory published, non-sensitive
follow-up questions are best handled on the
[EZHTML Discord](https://discord.gg/TQs6McKJJs) `#security` channel. The
private email remains the right channel for **new** disclosures only.
