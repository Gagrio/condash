## Description
condash: single-command ephemeral monitoring for Docker containers. Spins up a temporary Prometheus + Grafana stack against a target container.

## Goals
- Single binary
- Grafana dashboard filtered to the target container
- Clean shutdown (no orphaned containers)
- Works on openSUSE Leap 16

## Resources
- Rust
- Docker
- Leap 16 VM
- Existing Grafana dashboards to steal from
