# Gradio Browser Agent UI

This directory contains a Gradio-based user interface for interacting with the `agent-browser` and exploring its MCP endpoints.

## Features

- **Interactive Browser Agent:** Navigate to URLs, perform common actions (click, fill, hover, etc.), and view real-time screenshots and accessibility trees.
- **MCP Endpoints Overview:** Dynamically lists all available tools from the Rust MCP server, including their descriptions and parameter schemas.

## Setup

1. **Install Python dependencies:**
   ```bash
   pip install -r requirements.txt
   ```

2. **Ensure agent-browser is installed:**
   ```bash
   cargo install agent-browser && agent-browser install
   ```

3. **Build the MCP server (optional, for overview tab):**
   ```bash
   cd ../../mcp-server-agent-browser
   cargo build --release
   ```

## Running the UI

From this directory, run:
```bash
python app.py
```

The UI will be available at `http://127.0.0.1:7860`.
