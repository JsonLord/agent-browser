#!/bin/bash
set -e

# Repository configuration
REPO_URL="https://github.com/JsonLord/agent-browser.git"
BRANCH="expose-all-commands-mcp-4385636345694592551"

echo "🚀 Starting setup for agent-browser..."

# 1. Clone the repository
if [ -d "agent-browser" ]; then
    echo "⚠️ Directory 'agent-browser' already exists. Updating..."
    cd agent-browser
    git fetch origin
    git checkout $BRANCH
    git pull origin $BRANCH
else
    echo "📂 Cloning repository..."
    git clone -b $BRANCH $REPO_URL
    cd agent-browser
fi

# 2. Install Rust dependencies and the CLI
echo "🦀 Installing agent-browser CLI..."
cargo install --path cli/
agent-browser install

# 3. Build the MCP Server
echo "🛠️ Building MCP server..."
cd mcp-server-agent-browser
cargo build --release
MCP_BIN_PATH="$(pwd)/target/release/mcp-server-agent-browser"
cd ..

# 4. Set up Gradio UI requirements
echo "🐍 Setting up Python environment for Gradio UI..."
if command -v pip3 &>/dev/null; then
    pip3 install -r examples/gradio-browser/requirements.txt
elif command -v pip &>/dev/null; then
    pip install -r examples/gradio-browser/requirements.txt
else
    echo "❌ pip not found. Please install Python and pip."
    exit 1
fi

# 5. Provide instructions
echo ""
echo "✅ Setup complete!"
echo "--------------------------------------------------"
echo "🏃 To run the Gradio UI:"
echo "   export AGENT_BROWSER_MCP_PATH=$MCP_BIN_PATH"
echo "   python3 examples/gradio-browser/app.py"
echo ""
echo "🤖 To use the MCP server in Claude Desktop (add to config):"
echo "   {"
echo "     \"mcpServers\": {"
echo "       \"agent-browser\": {"
echo "         \"command\": \"$MCP_BIN_PATH\","
echo "         \"args\": []"
echo "       }"
echo "     }"
echo "   }"
echo "--------------------------------------------------"
