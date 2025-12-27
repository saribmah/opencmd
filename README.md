# OpenCMD

A macOS productivity app for orchestrating AI coding tools and integrations. Think Raycast, but with AI at the core.

## Features

- **Command Palette** - Fast, fuzzy-searchable launcher (Cmd+K)
- **Runner Extensions** - Spawn and manage CLI tools like Claude Code, OpenCode, Aider with PTY support
- **Integration Extensions** - Connect to APIs like GitHub, Linear, Jira via sandboxed WASM runtime
- **Unified Interface** - One place to access all your AI coding assistants

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Bun](https://bun.sh/) (v1.0+)
- macOS 12+ (for Tauri v2)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/opencmd.git
cd opencmd

# Install dependencies
bun install

# Run in development mode
bun run tauri dev
```

### Build for Production

```bash
bun run tauri build
```

## Project Structure

```
opencmd/
├── apps/desktop/           # Tauri desktop application
│   ├── src/                # React frontend
│   └── src-tauri/          # Rust backend (thin binding layer)
│
├── crates/                 # Rust workspace - core logic
│   ├── core/               # App orchestration, config, event bus
│   ├── extension-host/     # Extension loading, runtime, sandboxing
│   └── protocol/           # Shared types (Rust ↔ TypeScript)
│
├── extensions/             # Built-in extensions
│   ├── claude-code/        # Claude Code CLI runner
│   ├── opencode/           # OpenCode CLI runner
│   ├── shell/              # Terminal runner
│   └── github-pr/          # GitHub PR integration
│
└── packages/               # Shared TypeScript packages
    ├── ui/                 # React component library
    ├── extension-api/      # SDK for extension authors
    └── protocol/           # TypeScript types matching Rust
```

## Creating Extensions

### Runner Extension

Runner extensions spawn CLI processes with PTY support:

```json
{
  "id": "my-cli",
  "name": "My CLI Tool",
  "version": "1.0.0",
  "type": "runner",
  "runner": {
    "command": "my-cli",
    "args": ["--prompt", "{{prompt}}"],
    "interactive": true,
    "pty": true
  },
  "commands": [
    {
      "id": "run",
      "name": "Run My CLI",
      "keywords": ["my", "cli", "tool"]
    }
  ],
  "permissions": ["process:spawn", "pty"]
}
```

### Integration Extension

Integration extensions run in a sandboxed environment:

```typescript
// extensions/my-integration/src/index.ts
import { defineExtension } from '@opencmd/extension-api';

defineExtension({
  commands: [
    {
      id: 'fetch-data',
      async handler(ctx) {
        const response = await ctx.fetch('https://api.example.com/data');
        const data = await response.json();
        await ctx.showResults(data.items.map(item => ({
          title: item.name,
          subtitle: item.description
        })));
      }
    }
  ]
});
```

## Development

### Useful Commands

```bash
# Run desktop app in dev mode
bun run tauri dev

# Build TypeScript packages
bun run build:packages

# Build extensions
bun run build:extensions

# Check Rust code
cargo check

# Run Rust tests
cargo test

# Type check all TypeScript
bun run typecheck
```

### Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed documentation on how the system is designed.

## Tech Stack

- **Frontend**: React, TypeScript, Vite, cmdk
- **Backend**: Rust, Tauri v2
- **Extension Runtime**: PTY (runners), WASM (integrations)
- **Package Manager**: Bun (monorepo workspaces)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

MIT
