# condash 🐳📊

**Single-command ephemeral monitoring for Docker containers**

`condash` spins up a temporary Prometheus + Grafana monitoring stack for any Docker container, opens a dashboard in your browser, and tears everything down cleanly when you're done.

Perfect for quick debugging sessions, demos, or when you need instant visibility into container metrics without complex setup.

```bash
$ condash my-container
🔍 Validating container...
   ✓ Found container: my-container
🔌 Checking port availability...
   ✓ Ports 3000, 9090, 8080 are available
🚀 Starting monitoring stack...
   ✓ cAdvisor  ✓ Prometheus  ✓ Grafana
⏳ Waiting for Grafana...
🌐 Opening dashboard: http://localhost:3000/d/condash?var-container=my-container
Press Ctrl+C to stop monitoring and clean up.
```

## ✨ Features

- **🚀 Single command** - No configuration files, no setup
- **⚡ Fast startup** - Dashboard ready in ~10 seconds
- **🎯 Container-specific** - Automatically filtered to your target container
- **🧹 Self-cleaning** - No orphaned containers, guaranteed cleanup on any exit
- **📊 Essential metrics** - CPU, Memory, Network I/O, Disk I/O
- **🔒 Zero persistence** - Ephemeral stack with 1-hour data retention

## 📋 Prerequisites

- Docker (with `docker compose` command)
- Rust toolchain (for building from source)
- Linux or macOS
- Ports 3000, 8080, 9090 available

## 🔧 Installation

### Homebrew (macOS - Recommended)

```bash
brew install yourusername/condash/condash
```

### Download pre-built binary

Pre-built binaries are available for Linux and macOS:

```bash
# Linux ARM64 (aarch64) - like openSUSE Leap 16 on ARM
wget https://github.com/yourusername/condash/releases/latest/download/condash-linux-aarch64
chmod +x condash-linux-aarch64
sudo mv condash-linux-aarch64 /usr/local/bin/condash

# Linux x86_64
wget https://github.com/yourusername/condash/releases/latest/download/condash-linux-x86_64
chmod +x condash-linux-x86_64
sudo mv condash-linux-x86_64 /usr/local/bin/condash

# macOS Apple Silicon (M1/M2/M3)
curl -L https://github.com/yourusername/condash/releases/latest/download/condash-macos-aarch64 -o condash
chmod +x condash
sudo mv condash /usr/local/bin/condash
```

### Build from source

```bash
git clone https://github.com/yourusername/condash.git
cd condash
cargo build --release
sudo cp target/release/condash /usr/local/bin/
```

## 🚀 Usage

### Basic usage

```bash
# Start monitoring a container
condash container-name

# Example with nginx
condash my-nginx
```

### What happens

1. ✅ Validates the target container exists
2. ✅ Checks required ports are available (3000, 8080, 9090)
3. ✅ Creates temporary directory with embedded configs
4. ✅ Starts monitoring stack (cAdvisor + Prometheus + Grafana)
5. ✅ Waits for Grafana to be healthy
6. ✅ Opens dashboard in your default browser
7. ⏸️ Waits for Ctrl+C
8. ✅ Tears down all containers and cleans up

### Dashboard metrics

The dashboard shows real-time metrics for your container:

- **CPU Usage** - Percentage utilization over time
- **Memory Usage** - Memory consumption in bytes
- **Memory Cached** - Cached memory
- **Disk I/O** - Read/write operations per second
- **Network Traffic** - Received/transmitted bytes per second
- **Container Restarts** - Heatmap of restart events

All panels show the last 5 minutes by default with 5-second refresh intervals.

> ⏱️ **Note:** Metrics take ~30-60 seconds to start appearing in the graphs. The dashboard queries use rate calculations over 10-minute windows, so you'll see flatlines initially. Idle containers will show flatlines - generate some load to see activity!

## 🎯 Example Scenarios

### Debug a production issue

```bash
# Container acting strange? Monitor it instantly
condash production-api

# Generate some load to see what's happening
# Watch the dashboard for CPU spikes, memory leaks, etc.
```

### Demo container resource usage

```bash
# Great for presentations
condash demo-app

# Dashboard opens automatically - perfect for showing metrics live
```

### Quick health check

```bash
# Is my container actually doing work?
condash background-worker

# Flatlines = idle, activity = working
```

## ⚠️ Troubleshooting

### "Container not found"
```bash
# Make sure the container exists and is running
docker ps | grep your-container
```

### "Ports already in use"
```bash
# Stop any services using ports 3000, 8080, or 9090
sudo lsof -i :3000
sudo lsof -i :8080
sudo lsof -i :9090
```

### "Dashboard not found"
```bash
# Usually resolves after a few seconds - Grafana needs time to provision
# Wait 10-15 seconds and refresh your browser
```

### SELinux permission issues
If you see permission denied errors in logs, ensure the docker-compose.yaml has `:z` flags on volume mounts (already included in this version).

## 🏗️ Architecture

```
┌─────────────────┐
│    condash      │  Single Rust binary
└────────┬────────┘
         │
         ├─► Validates container
         ├─► Creates temp dir with configs
         ├─► Starts docker compose stack
         │
         ▼
┌────────────────────────────────────┐
│  Temporary Monitoring Stack        │
│                                    │
│  ┌──────────┐   ┌────────────┐   │
│  │ cAdvisor │──▶│ Prometheus │   │
│  └──────────┘   └──────┬─────┘   │
│                         │          │
│                         ▼          │
│                  ┌──────────┐     │
│                  │ Grafana  │     │
│                  └──────────┘     │
└────────────────────────────────────┘
         │
         ▼
    Your Browser 🌐
```

## 🔒 Security Notes

- ⚠️ Grafana runs with **anonymous admin access** (ephemeral use only!)
- ⚠️ cAdvisor requires **privileged mode** to access container metrics
- ✅ All containers run on an isolated Docker network
- ✅ Only Grafana port (3000) is exposed to host
- ✅ Metrics stored for max 1 hour, then auto-deleted

**Not recommended for production monitoring** - use proper monitoring solutions like Prometheus + Grafana with authentication.

## 🛠️ Development

### Project structure

```
condash/
├── src/
│   ├── main.rs          # CLI and main flow
│   ├── docker.rs        # Docker operations
│   ├── config.rs        # Config file management
│   ├── health.rs        # Health checking
│   └── browser.rs       # Browser integration
├── configs/
│   ├── docker-compose.yaml
│   ├── prometheus.yaml
│   └── grafana/
│       ├── provisioning/
│       └── dashboards/
└── Cargo.toml
```

### Running tests

```bash
# Start a test container
docker run -d --name test-nginx nginx

# Run condash
cargo run -- test-nginx

# Clean up
docker rm -f test-nginx
```

## 📝 License

Apache License 2.0 - see [LICENSE](LICENSE) for details.

## 🙏 Credits

- Dashboard based on [Grafana Labs Dashboard 19908](https://grafana.com/grafana/dashboards/19908)
- Built for [SUSE Hackweek 2025](https://hackweek.opensuse.org/)
- Powered by [cAdvisor](https://github.com/google/cadvisor), [Prometheus](https://prometheus.io/), and [Grafana](https://grafana.com/)

## 🤝 Contributing

Contributions welcome! Please open an issue or PR.

### Ideas for future enhancements

- [ ] `--all` flag to monitor all containers
- [ ] `--compare` flag for side-by-side container comparison
- [ ] Podman support
- [ ] Custom port configuration
- [ ] TUI mode for headless environments
- [ ] Export metrics to file
- [ ] RPM packaging for easy installation

---

**Made with 🦀 Rust and ❤️ for quick container debugging**