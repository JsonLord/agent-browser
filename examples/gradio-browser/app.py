import gradio as gr
import subprocess
import json
import os
import base64
from PIL import Image
import io
import asyncio
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

# Path to the agent-browser binary
AGENT_BROWSER_PATH = os.environ.get("AGENT_BROWSER_PATH", "agent-browser")

def run_command(args):
    try:
        result = subprocess.run(
            [AGENT_BROWSER_PATH] + args + ["--json"],
            capture_output=True,
            text=True,
            check=True
        )
        return json.loads(result.stdout)
    except subprocess.CalledProcessError as e:
        try:
            return json.loads(e.stdout)
        except:
            return {"success": False, "error": e.stderr or str(e)}
    except Exception as e:
        return {"success": False, "error": str(e)}

def get_screenshot(session_id="default"):
    resp = run_command(["--session", session_id, "screenshot"])
    if resp.get("success"):
        path = resp.get("data", {}).get("path")
        if path and os.path.exists(path):
            return Image.open(path)
    return None

def get_snapshot(session_id="default"):
    resp = run_command(["--session", session_id, "snapshot", "-i"])
    if resp.get("success"):
        return resp.get("data", {}).get("snapshot", "")
    return resp.get("error", "Failed to get snapshot")

def navigate(url, session_id):
    resp = run_command(["--session", session_id, "open", url])
    if not resp.get("success"):
        return None, resp.get("error", "Navigation failed"), ""

    img = get_screenshot(session_id)
    snap = get_snapshot(session_id)
    return img, "Navigated to " + url, snap

def perform_action(action, selector, text, session_id):
    args = ["--session", session_id, action, selector]
    if action in ["fill", "type"] and text:
        args.append(text)

    resp = run_command(args)
    if not resp.get("success"):
        return None, resp.get("error", "Action failed"), ""

    img = get_screenshot(session_id)
    snap = get_snapshot(session_id)
    return img, f"Performed {action} on {selector}", snap

async def list_mcp_tools():
    # Attempt to start the Rust MCP server and list its tools
    # Assuming it's built and available at a known path or via AGENT_BROWSER_MCP_PATH
    mcp_bin = os.environ.get("AGENT_BROWSER_MCP_PATH", "./mcp-server-agent-browser/target/release/mcp-server-agent-browser")
    if not os.path.exists(mcp_bin):
        # Fallback to dev path
        mcp_bin = "./mcp-server-agent-browser/target/debug/mcp-server-agent-browser"

    if not os.path.exists(mcp_bin):
        return "MCP server binary not found. Please build it first."

    server_params = StdioServerParameters(
        command=mcp_bin,
        args=[],
        env=None
    )

    try:
        async with stdio_client(server_params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                tools = await session.list_tools()

                output = "# Available MCP Endpoints\n\n"
                for tool in tools.tools:
                    output += f"### {tool.name}\n"
                    output += f"{tool.description}\n\n"
                    output += f"**Parameters:**\n```json\n{json.dumps(tool.inputSchema, indent=2)}\n```\n\n"
                return output
    except Exception as e:
        return f"Error connecting to MCP server: {str(e)}"

def get_tools_sync():
    return asyncio.run(list_mcp_tools())

with gr.Blocks(title="Agent Browser Interface") as demo:
    gr.Markdown("# 🌐 Agent Browser UI")

    with gr.Tab("Browser Agent"):
        with gr.Row():
            with gr.Column(scale=2):
                url_input = gr.Textbox(label="URL", placeholder="https://example.com")
                session_input = gr.Textbox(label="Session ID", value="default")
                nav_btn = gr.Button("Navigate", variant="primary")

                with gr.Group():
                    gr.Markdown("### Actions")
                    with gr.Row():
                        action_type = gr.Dropdown(
                            choices=["click", "fill", "type", "hover", "focus", "check", "uncheck"],
                            value="click",
                            label="Action"
                        )
                        selector_input = gr.Textbox(label="Selector (@ref or CSS)", placeholder="@e1")
                    text_input = gr.Textbox(label="Text (for fill/type)", placeholder="Enter text...")
                    action_btn = gr.Button("Perform Action")

                status_output = gr.Textbox(label="Status")

            with gr.Column(scale=3):
                screenshot_output = gr.Image(label="Browser View", type="pil")
                snapshot_output = gr.Code(label="Accessibility Tree (Snapshot)", language="markdown")

    with gr.Tab("MCP Endpoints Overview"):
        refresh_btn = gr.Button("Fetch/Refresh MCP Tools")
        mcp_overview = gr.Markdown("Click the button to load available tools from the MCP server.")

    # Event handlers
    nav_btn.click(
        fn=navigate,
        inputs=[url_input, session_input],
        outputs=[screenshot_output, status_output, snapshot_output]
    )

    action_btn.click(
        fn=perform_action,
        inputs=[action_type, selector_input, text_input, session_input],
        outputs=[screenshot_output, status_output, snapshot_output]
    )

    refresh_btn.click(
        fn=get_tools_sync,
        inputs=[],
        outputs=[mcp_overview]
    )

if __name__ == "__main__":
    demo.launch()
