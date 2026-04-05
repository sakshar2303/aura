# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in the Aura compiler or any generated output, please report it responsibly:

1. **Do NOT open a public issue**
2. Email: dev@360labs.ai
3. Include: description, reproduction steps, and impact assessment

We will respond within 48 hours and provide a fix timeline.

## Scope

Security issues we care about:
- **Compiler bugs** that generate insecure code (XSS, injection, etc.)
- **Security type bypasses** — ways to circumvent `secret`/`sanitized`/`token` enforcement
- **Package manager** — supply chain, signature verification issues
- **Agent API** — unauthorized access, injection via JSON-RPC

## Security Types

Aura's security types (`secret`, `sanitized`, `email`, `url`, `token`) are compile-time enforced. If you find a way to bypass these checks, that's a critical security issue.
