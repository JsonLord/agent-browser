use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::*,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::executor::{exec_browser, validate_file_path, validate_session_id};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn run(args: Vec<String>) -> Result<CallToolResult, McpError> {
    run_with_timeout(args, None).await
}

async fn run_with_timeout(args: Vec<String>, timeout_secs: Option<u64>) -> Result<CallToolResult, McpError> {
    match exec_browser(args, timeout_secs).await {
        Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
    }
}

async fn run_with_stdin(args: Vec<String>, stdin: String) -> Result<CallToolResult, McpError> {
    match crate::executor::exec_browser_with_stdin(args, None, Some(stdin)).await {
        Ok(text) => Ok(CallToolResult::success(vec![Content::text(text)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
    }
}

fn validated_path(path: &str) -> Result<(), McpError> {
    validate_file_path(path).map_err(|e| McpError::invalid_params(e, None))
}

fn session_args(session_id: Option<&str>) -> Result<Vec<String>, McpError> {
    match session_id {
        Some(sid) => {
            validate_session_id(sid).map_err(|e| {
                McpError::invalid_params(e, None)
            })?;
            Ok(vec!["--session".into(), sid.into()])
        }
        None => Ok(vec![]),
    }
}

// ---------------------------------------------------------------------------
// Parameter structs
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SessionOnly {
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StreamEnableParams {
    pub port: Option<u16>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetValueParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetCountParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetBoxParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetStylesParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UrlParams {
    #[schemars(description = "The URL to navigate to")]
    pub url: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectorParams {
    #[schemars(description = "CSS selector, text, or @ref from snapshot")]
    pub selector: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectorValueParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Value to enter")]
    pub value: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TypeTextParams {
    #[schemars(description = "Selector for the input element")]
    pub selector: String,
    #[schemars(description = "Text to type")]
    pub text: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KeyParams {
    #[schemars(description = "Key to press (e.g. 'Enter', 'Escape', 'Tab', 'Control+a')")]
    pub key: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KeyboardTypeParams {
    #[schemars(description = "Text to type with real keystrokes")]
    pub text: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollParams {
    #[schemars(description = "Scroll direction")]
    pub direction: ScrollDirection,
    #[schemars(description = "Scroll amount in pixels")]
    pub amount: Option<u32>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DragParams {
    #[schemars(description = "Selector for source element")]
    pub source: String,
    #[schemars(description = "Selector for destination element")]
    pub destination: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UploadParams {
    #[schemars(description = "Selector for the file input element")]
    pub selector: String,
    #[schemars(description = "File paths to upload")]
    pub files: Vec<String>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DownloadParams {
    #[schemars(description = "Selector for the element to click to trigger download")]
    pub selector: String,
    #[schemars(description = "File path to save the download")]
    pub path: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct OptionalSelectorParams {
    #[schemars(description = "Selector for the element (full page if not provided)")]
    pub selector: Option<String>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetHtmlParams {
    #[schemars(description = "Selector for the element (full page if not provided)")]
    pub selector: Option<String>,
    #[schemars(description = "Get outer HTML instead of inner HTML")]
    pub outer: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetAttrParams {
    #[schemars(description = "Selector for the element")]
    pub selector: String,
    #[schemars(description = "Name of the attribute to get")]
    pub attribute: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SnapshotParams {
    #[schemars(description = "Only show interactive elements")]
    pub interactive: Option<bool>,
    #[schemars(description = "Remove empty structural elements")]
    pub compact: Option<bool>,
    #[schemars(description = "Limit tree depth")]
    pub depth: Option<u32>,
    #[schemars(description = "Scope to a CSS selector")]
    pub selector: Option<String>,
    #[schemars(description = "Return JSON output")]
    pub json: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScreenshotParams {
    #[schemars(description = "File path to save the screenshot")]
    pub path: Option<String>,
    #[schemars(description = "Selector for element to screenshot")]
    pub selector: Option<String>,
    #[schemars(description = "Capture the full scrollable page")]
    pub full_page: Option<bool>,
    #[schemars(description = "Add numbered labels for vision models")]
    pub annotate: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PdfParams {
    #[schemars(description = "File path to save the PDF")]
    pub path: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewSessionParams {
    #[schemars(description = "Viewport width in pixels")]
    pub width: Option<u32>,
    #[schemars(description = "Viewport height in pixels")]
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CloseSessionParams {
    #[schemars(description = "Session ID to close")]
    pub session_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum WaitState {
    Attached,
    Detached,
    Visible,
    Hidden,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WaitParams {
    #[schemars(description = "Selector to wait for, or milliseconds (e.g. '5000')")]
    pub selector: Option<String>,
    #[schemars(description = "Wait for URL pattern")]
    pub url: Option<String>,
    #[schemars(description = "Wait for load state (load, domcontentloaded, networkidle)")]
    pub load: Option<String>,
    #[schemars(description = "Wait for JS function to return true")]
    pub fn_expr: Option<String>,
    #[schemars(description = "Wait for text to appear")]
    pub text: Option<String>,
    #[schemars(description = "Wait for download to complete (optionally specify path)")]
    pub download: Option<String>,
    #[schemars(description = "Maximum wait time in milliseconds")]
    pub timeout: Option<u64>,
    #[schemars(description = "Element state to wait for (with selector)")]
    pub state: Option<WaitState>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FindParams {
    #[schemars(
        description = "Locator type (role, text, label, placeholder, alt, title, testid, first, last)"
    )]
    pub locator: String,
    #[schemars(description = "Value to search for")]
    pub value: String,
    #[schemars(
        description = "Action to perform (click, fill, type, hover, focus, check, uncheck, text)"
    )]
    pub action: Option<String>,
    #[schemars(description = "Text to enter (for fill/type actions)")]
    pub text: Option<String>,
    #[schemars(description = "Filter by accessible name (for 'role' locator)")]
    pub name: Option<String>,
    #[schemars(description = "Require exact text match")]
    pub exact: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FindNthParams {
    #[schemars(description = "Index (0 for first, -1 for last)")]
    pub index: i32,
    #[schemars(description = "CSS selector")]
    pub selector: String,
    #[schemars(description = "Action to perform")]
    pub action: Option<String>,
    #[schemars(description = "Text to enter")]
    pub text: Option<String>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MouseMoveParams {
    pub x: i32,
    pub y: i32,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MouseButtonParams {
    #[schemars(description = "Mouse button (left, right, middle)")]
    pub button: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MouseWheelParams {
    #[schemars(description = "Vertical scroll amount")]
    pub dy: Option<i32>,
    #[schemars(description = "Horizontal scroll amount")]
    pub dx: Option<i32>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetDeviceParams {
    #[schemars(description = "Device name (e.g. 'iPhone 14')")]
    pub device: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetGeoParams {
    pub latitude: f64,
    pub longitude: f64,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetOfflineParams {
    pub offline: bool,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetHeadersParams {
    #[schemars(description = "HTTP headers as JSON object")]
    pub headers: serde_json::Value,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetCredentialsParams {
    pub username: String,
    pub password: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetMediaParams {
    #[schemars(description = "Color scheme (dark, light, no-preference)")]
    pub color_scheme: Option<String>,
    #[schemars(description = "Reduced motion (reduce, no-preference)")]
    pub reduced_motion: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AuthSaveParams {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: Option<String>,
    pub password_stdin: Option<bool>,
    pub username_selector: Option<String>,
    pub password_selector: Option<String>,
    pub submit_selector: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AuthNameParams {
    pub name: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NetworkRouteParams {
    pub url: String,
    pub abort: Option<bool>,
    pub body: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NetworkUnrouteParams {
    pub url: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NetworkRequestsParams {
    pub filter: Option<String>,
    pub clear: Option<bool>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NetworkHarParams {
    pub path: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StorageGetParams {
    #[schemars(description = "Storage type (local, session)")]
    pub r#type: String,
    pub key: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StorageSetParams {
    #[schemars(description = "Storage type (local, session)")]
    pub r#type: String,
    pub key: String,
    pub value: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StorageClearParams {
    #[schemars(description = "Storage type (local, session)")]
    pub r#type: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabNewParams {
    pub url: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabSwitchParams {
    pub index: i32,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabCloseParams {
    pub index: Option<i32>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FrameParams {
    pub selector: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DialogParams {
    pub text: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TraceParams {
    pub path: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProfilerStartParams {
    pub categories: Option<Vec<String>>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RecordStartParams {
    pub path: String,
    pub url: Option<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConsoleParams {
    pub clear: Option<bool>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConfirmationParams {
    pub confirmation_id: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SwipeParams {
    pub direction: String,
    pub distance: Option<u32>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DiffSnapshotParams {
    pub baseline: Option<String>,
    pub selector: Option<String>,
    pub compact: Option<bool>,
    pub depth: Option<u32>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DiffScreenshotParams {
    pub baseline: String,
    pub output: Option<String>,
    pub threshold: Option<f64>,
    pub selector: Option<String>,
    pub full_page: Option<bool>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DiffUrlParams {
    pub url1: String,
    pub url2: String,
    pub screenshot: Option<bool>,
    pub full_page: Option<bool>,
    pub wait_until: Option<String>,
    pub selector: Option<String>,
    pub compact: Option<bool>,
    pub depth: Option<u32>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BatchParams {
    pub commands: Vec<Vec<String>>,
    pub bail: Option<bool>,
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CookieGetParams {
    #[schemars(description = "URLs to get cookies for")]
    pub urls: Option<Vec<String>>,
    #[schemars(description = "Return JSON output")]
    pub json: Option<bool>,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CookieSetParams {
    #[schemars(description = "Cookies to set as JSON")]
    pub cookies: serde_json::Value,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EvalParams {
    #[schemars(description = "JavaScript code to execute")]
    pub script: String,
    #[schemars(description = "Browser session ID")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConnectParams {
    #[schemars(description = "CDP port number or WebSocket URL")]
    pub target: String,
}


// ---------------------------------------------------------------------------
// Server
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct AgentBrowserServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl AgentBrowserServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    // ── Navigation ──────────────────────────────────────────────────────

    #[tool(description = "Navigate to a URL in the browser")]
    async fn browser_navigate(
        &self,
        Parameters(p): Parameters<UrlParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["open".into(), p.url];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get the WebSocket URL for Chrome DevTools Protocol")]
    async fn browser_get_cdp_url(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "cdp-url".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Navigate back in browser history")]
    async fn browser_go_back(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["back".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Navigate forward in browser history")]
    async fn browser_go_forward(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["forward".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Reload the current page")]
    async fn browser_reload(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["reload".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Interaction ─────────────────────────────────────────────────────

    #[tool(description = "Click on an element by selector or @ref from snapshot")]
    async fn browser_click(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["click".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Double-click on an element")]
    async fn browser_dblclick(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["dblclick".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Fill a text input field (clears existing value first)")]
    async fn browser_fill(
        &self,
        Parameters(p): Parameters<SelectorValueParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["fill".into(), p.selector, p.value];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Type text character by character (triggers key events)")]
    async fn browser_type(
        &self,
        Parameters(p): Parameters<TypeTextParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["type".into(), p.selector, p.text];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Type text with real keystrokes without targeting an element")]
    async fn browser_keyboard_type(
        &self,
        Parameters(p): Parameters<KeyboardTypeParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["keyboard".into(), "type".into(), p.text];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Insert text without key events (no selector needed)")]
    async fn browser_keyboard_inserttext(
        &self,
        Parameters(p): Parameters<KeyboardTypeParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["keyboard".into(), "inserttext".into(), p.text];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Press a keyboard key")]
    async fn browser_press(
        &self,
        Parameters(p): Parameters<KeyParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["press".into(), p.key];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Hover over an element")]
    async fn browser_hover(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["hover".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Focus an element")]
    async fn browser_focus(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["focus".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Hold a keyboard key down")]
    async fn browser_keydown(
        &self,
        Parameters(p): Parameters<KeyParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["keydown".into(), p.key];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Release a keyboard key")]
    async fn browser_keyup(
        &self,
        Parameters(p): Parameters<KeyParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["keyup".into(), p.key];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Move the mouse to a specific coordinate")]
    async fn browser_mouse_move(
        &self,
        Parameters(p): Parameters<MouseMoveParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["mouse".into(), "move".into(), p.x.to_string(), p.y.to_string()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Press a mouse button")]
    async fn browser_mouse_down(
        &self,
        Parameters(p): Parameters<MouseButtonParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["mouse".into(), "down".into()];
        if let Some(btn) = p.button {
            a.push(btn);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Release a mouse button")]
    async fn browser_mouse_up(
        &self,
        Parameters(p): Parameters<MouseButtonParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["mouse".into(), "up".into()];
        if let Some(btn) = p.button {
            a.push(btn);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Scroll the mouse wheel")]
    async fn browser_mouse_wheel(
        &self,
        Parameters(p): Parameters<MouseWheelParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["mouse".into(), "wheel".into()];
        if let Some(dy) = p.dy {
            a.push(dy.to_string());
        }
        if let Some(dx) = p.dx {
            a.push(dx.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Scroll the page in a direction")]
    async fn browser_scroll(
        &self,
        Parameters(p): Parameters<ScrollParams>,
    ) -> Result<CallToolResult, McpError> {
        let dir = match p.direction {
            ScrollDirection::Up => "up",
            ScrollDirection::Down => "down",
            ScrollDirection::Left => "left",
            ScrollDirection::Right => "right",
        };
        let mut a = vec!["scroll".into(), dir.into()];
        if let Some(px) = p.amount {
            a.push(px.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Scroll an element into view")]
    async fn browser_scroll_into_view(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["scrollintoview".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Select an option from a dropdown")]
    async fn browser_select(
        &self,
        Parameters(p): Parameters<SelectorValueParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["select".into(), p.selector, p.value];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Check a checkbox or radio button")]
    async fn browser_check(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["check".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Uncheck a checkbox")]
    async fn browser_uncheck(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["uncheck".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Find elements using semantic locators and perform an action")]
    async fn browser_find(
        &self,
        Parameters(p): Parameters<FindParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["find".into(), p.locator, p.value];
        if let Some(act) = p.action {
            a.push(act);
        }
        if let Some(txt) = p.text {
            a.push(txt);
        }
        if let Some(name) = p.name {
            a.push("--name".into());
            a.push(name);
        }
        if p.exact.unwrap_or(false) {
            a.push("--exact".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Find the Nth element and perform an action")]
    async fn browser_find_nth(
        &self,
        Parameters(p): Parameters<FindNthParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["find".into(), "nth".into(), p.index.to_string(), p.selector];
        if let Some(act) = p.action {
            a.push(act);
        }
        if let Some(txt) = p.text {
            a.push(txt);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Drag an element to another element")]
    async fn browser_drag(
        &self,
        Parameters(p): Parameters<DragParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["drag".into(), p.source, p.destination];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Upload files to a file input element")]
    async fn browser_upload(
        &self,
        Parameters(p): Parameters<UploadParams>,
    ) -> Result<CallToolResult, McpError> {
        for f in &p.files {
            validated_path(f)?;
        }
        let mut a = vec!["upload".into(), p.selector];
        a.extend(p.files);
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Download a file by clicking an element")]
    async fn browser_download(
        &self,
        Parameters(p): Parameters<DownloadParams>,
    ) -> Result<CallToolResult, McpError> {
        validated_path(&p.path)?;
        let mut a = vec!["download".into(), p.selector, p.path];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Information Retrieval ───────────────────────────────────────────

    #[tool(description = "Get text content from an element or the entire page")]
    async fn browser_get_text(
        &self,
        Parameters(p): Parameters<OptionalSelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "text".into()];
        if let Some(sel) = p.selector {
            a.push(sel);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get input value from an element")]
    async fn browser_get_value(
        &self,
        Parameters(p): Parameters<GetValueParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "value".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Count elements matching a selector")]
    async fn browser_get_count(
        &self,
        Parameters(p): Parameters<GetCountParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "count".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get bounding box of an element")]
    async fn browser_get_box(
        &self,
        Parameters(p): Parameters<GetBoxParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "box".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get computed styles of an element")]
    async fn browser_get_styles(
        &self,
        Parameters(p): Parameters<GetStylesParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "styles".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get HTML content from an element or the entire page")]
    async fn browser_get_html(
        &self,
        Parameters(p): Parameters<GetHtmlParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "html".into()];
        if let Some(sel) = p.selector {
            a.push(sel);
        }
        if p.outer.unwrap_or(false) {
            a.push("--outer".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get an attribute value from an element")]
    async fn browser_get_attribute(
        &self,
        Parameters(p): Parameters<GetAttrParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "attr".into(), p.selector, p.attribute];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get the current page URL")]
    async fn browser_get_url(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "url".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get the current page title")]
    async fn browser_get_title(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["get".into(), "title".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get an accessibility tree snapshot of the page with @refs for AI interaction")]
    async fn browser_snapshot(
        &self,
        Parameters(p): Parameters<SnapshotParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["snapshot".into()];
        if p.interactive.unwrap_or(false) {
            a.push("-i".into());
        }
        if p.compact.unwrap_or(false) {
            a.push("-c".into());
        }
        if let Some(d) = p.depth {
            a.push("-d".into());
            a.push(d.to_string());
        }
        if let Some(sel) = p.selector {
            a.push("-s".into());
            a.push(sel);
        }
        if p.json.unwrap_or(false) {
            a.push("--json".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Element State ───────────────────────────────────────────────────

    #[tool(description = "Check if an element is visible")]
    async fn browser_is_visible(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["is".into(), "visible".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Check if an element is enabled")]
    async fn browser_is_enabled(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["is".into(), "enabled".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Check if a checkbox/radio is checked")]
    async fn browser_is_checked(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["is".into(), "checked".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Screenshot & PDF ────────────────────────────────────────────────

    #[tool(description = "Take a screenshot of the page or an element")]
    async fn browser_screenshot(
        &self,
        Parameters(p): Parameters<ScreenshotParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["screenshot".into()];
        if let Some(ref path) = p.path {
            validated_path(path)?;
            a.push(path.clone());
        }
        if let Some(sel) = p.selector {
            a.push("--selector".into());
            a.push(sel);
        }
        if p.full_page.unwrap_or(false) {
            a.push("--full".into());
        }
        if p.annotate.unwrap_or(false) {
            a.push("--annotate".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Generate a PDF of the current page")]
    async fn browser_pdf(
        &self,
        Parameters(p): Parameters<PdfParams>,
    ) -> Result<CallToolResult, McpError> {
        validated_path(&p.path)?;
        let mut a = vec!["pdf".into(), p.path];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Session Management ──────────────────────────────────────────────

    #[tool(description = "Create a new isolated browser session")]
    async fn browser_new_session(
        &self,
        Parameters(p): Parameters<NewSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["session".into(), "new".into()];
        if let Some(w) = p.width {
            a.push("--width".into());
            a.push(w.to_string());
        }
        if let Some(h) = p.height {
            a.push("--height".into());
            a.push(h.to_string());
        }
        run(a).await
    }

    #[tool(description = "Close a specific browser session by its ID (use browser_close to close the entire browser)")]
    async fn browser_close_session(
        &self,
        Parameters(p): Parameters<CloseSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        validate_session_id(&p.session_id).map_err(|e| {
            McpError::invalid_params(e, None)
        })?;
        run(vec![
            "close".into(),
            "--session".into(),
            p.session_id,
        ]).await
    }

    // ── Wait ────────────────────────────────────────────────────────────

    #[tool(description = "Wait for an element, URL, load state, text, JS condition, or download")]
    async fn browser_wait(
        &self,
        Parameters(p): Parameters<WaitParams>,
    ) -> Result<CallToolResult, McpError> {
        // Derive executor timeout from wait timeout so long waits aren't killed prematurely
        let executor_timeout = p.timeout.map(|ms| (ms / 1000) + 10);
        let mut a = vec!["wait".into()];

        if let Some(url) = p.url {
            a.push("--url".into());
            a.push(url);
        } else if let Some(load) = p.load {
            a.push("--load".into());
            a.push(load);
        } else if let Some(expr) = p.fn_expr {
            a.push("--fn".into());
            a.push(expr);
        } else if let Some(text) = p.text {
            a.push("--text".into());
            a.push(text);
        } else if let Some(download) = p.download {
            a.push("--download".into());
            if !download.is_empty() {
                a.push(download);
            }
        } else if let Some(sel) = p.selector {
            a.push(sel);
        } else {
            return Err(McpError::invalid_params(
                "Missing wait criteria (selector, url, load, fn_expr, text, or download)",
                None,
            ));
        }

        if let Some(t) = p.timeout {
            a.push("--timeout".into());
            a.push(t.to_string());
        }
        if let Some(s) = p.state {
            let state_str = match s {
                WaitState::Attached => "attached",
                WaitState::Detached => "detached",
                WaitState::Visible => "visible",
                WaitState::Hidden => "hidden",
            };
            a.push("--state".into());
            a.push(state_str.into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run_with_timeout(a, executor_timeout).await
    }

    // ── Cookies ─────────────────────────────────────────────────────────

    #[tool(description = "Get cookies from the browser")]
    async fn browser_get_cookies(
        &self,
        Parameters(p): Parameters<CookieGetParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["cookies".into(), "get".into()];
        if p.json.unwrap_or(false) {
            a.push("--json".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        if let Some(urls) = p.urls {
            a.push("--".into());
            a.extend(urls);
        }
        run(a).await
    }

    #[tool(description = "Set cookies in the browser")]
    async fn browser_set_cookies(
        &self,
        Parameters(p): Parameters<CookieSetParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["cookies".into(), "set".into(), p.cookies.to_string()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Clear all cookies")]
    async fn browser_clear_cookies(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["cookies".into(), "clear".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── JavaScript ──────────────────────────────────────────────────────

    #[tool(description = "Execute JavaScript in the browser context and return the result")]
    async fn browser_evaluate(
        &self,
        Parameters(p): Parameters<EvalParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["eval".into(), p.script];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Console & Network ───────────────────────────────────────────────

    #[tool(description = "Get console log messages from the browser")]
    async fn browser_get_console(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["console".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Get network requests captured by the browser")]
    async fn browser_get_network(
        &self,
        Parameters(p): Parameters<NetworkRequestsParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["network".into(), "requests".into()];
        if let Some(f) = p.filter {
            a.push("--filter".into());
            a.push(f);
        }
        if p.clear.unwrap_or(false) {
            a.push("--clear".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Connect ─────────────────────────────────────────────────────────

    #[tool(description = "Connect to a running browser via Chrome DevTools Protocol")]
    async fn browser_connect(
        &self,
        Parameters(p): Parameters<ConnectParams>,
    ) -> Result<CallToolResult, McpError> {
        run(vec!["connect".into(), p.target]).await
    }

    // ── Close ───────────────────────────────────────────────────────────

    #[tool(description = "Close the entire browser daemon and all sessions")]
    async fn browser_close(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["close".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Browser Settings ────────────────────────────────────────────────

    #[tool(description = "Set the browser viewport size and scale")]
    async fn browser_set_viewport(
        &self,
        Parameters(p): Parameters<NewSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["set".into(), "viewport".into()];
        if let (Some(w), Some(h)) = (p.width, p.height) {
            a.push(w.to_string());
            a.push(h.to_string());
        } else {
            return Err(McpError::invalid_params("Width and height are required", None));
        }
        run(a).await
    }

    #[tool(description = "Emulate a specific device (viewport + user agent)")]
    async fn browser_set_device(
        &self,
        Parameters(p): Parameters<SetDeviceParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["set".into(), "device".into(), p.device];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Set geolocation coordinates")]
    async fn browser_set_geo(
        &self,
        Parameters(p): Parameters<SetGeoParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec![
            "set".into(),
            "geo".into(),
            p.latitude.to_string(),
            p.longitude.to_string(),
        ];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Toggle offline mode")]
    async fn browser_set_offline(
        &self,
        Parameters(p): Parameters<SetOfflineParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec![
            "set".into(),
            "offline".into(),
            if p.offline { "on" } else { "off" }.into(),
        ];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Set global HTTP headers")]
    async fn browser_set_headers(
        &self,
        Parameters(p): Parameters<SetHeadersParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["set".into(), "headers".into(), p.headers.to_string()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Set HTTP basic authentication credentials")]
    async fn browser_set_credentials(
        &self,
        Parameters(p): Parameters<SetCredentialsParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["set".into(), "credentials".into(), p.username, p.password];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Emulate color scheme and motion preference")]
    async fn browser_set_media(
        &self,
        Parameters(p): Parameters<SetMediaParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["set".into(), "media".into()];
        if let Some(cs) = p.color_scheme {
            a.push(cs);
        }
        if let Some(rm) = p.reduced_motion {
            a.push(rm);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Authentication Vault ────────────────────────────────────────────

    #[tool(description = "Save credentials to the authentication vault")]
    async fn browser_auth_save(
        &self,
        Parameters(p): Parameters<AuthSaveParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["auth".into(), "save".into(), p.name, "--url".into(), p.url, "--username".into(), p.username];
        if let Some(pass) = p.password {
            a.push("--password".into());
            a.push(pass);
        }
        if p.password_stdin.unwrap_or(false) {
            a.push("--password-stdin".into());
        }
        if let Some(us) = p.username_selector {
            a.push("--username-selector".into());
            a.push(us);
        }
        if let Some(ps) = p.password_selector {
            a.push("--password-selector".into());
            a.push(ps);
        }
        if let Some(ss) = p.submit_selector {
            a.push("--submit-selector".into());
            a.push(ss);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Login using credentials from the vault")]
    async fn browser_auth_login(
        &self,
        Parameters(p): Parameters<AuthNameParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["auth".into(), "login".into(), p.name];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "List all profiles in the authentication vault")]
    async fn browser_auth_list(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["auth".into(), "list".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Show details of a vault profile (excluding password)")]
    async fn browser_auth_show(
        &self,
        Parameters(p): Parameters<AuthNameParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["auth".into(), "show".into(), p.name];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Delete a profile from the vault")]
    async fn browser_auth_delete(
        &self,
        Parameters(p): Parameters<AuthNameParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["auth".into(), "delete".into(), p.name];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Network Control ─────────────────────────────────────────────────

    #[tool(description = "Intercept or block network requests")]
    async fn browser_network_route(
        &self,
        Parameters(p): Parameters<NetworkRouteParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["network".into(), "route".into(), p.url];
        if p.abort.unwrap_or(false) {
            a.push("--abort".into());
        }
        if let Some(body) = p.body {
            a.push("--body".into());
            a.push(body);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Remove network intercept routes")]
    async fn browser_network_unroute(
        &self,
        Parameters(p): Parameters<NetworkUnrouteParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["network".into(), "unroute".into()];
        if let Some(url) = p.url {
            a.push(url);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Start recording network activity to a HAR file")]
    async fn browser_network_har_start(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["network".into(), "har".into(), "start".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Stop HAR recording and save to file")]
    async fn browser_network_har_stop(
        &self,
        Parameters(p): Parameters<NetworkHarParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["network".into(), "har".into(), "stop".into()];
        if let Some(ref path) = p.path {
            validated_path(path)?;
            a.push(path.clone());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Storage Management ──────────────────────────────────────────────

    #[tool(description = "Get localStorage or sessionStorage items")]
    async fn browser_storage_get(
        &self,
        Parameters(p): Parameters<StorageGetParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["storage".into(), p.r#type];
        if let Some(key) = p.key {
            a.push(key);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Set a storage item")]
    async fn browser_storage_set(
        &self,
        Parameters(p): Parameters<StorageSetParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["storage".into(), p.r#type, "set".into(), p.key, p.value];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Clear all items from storage")]
    async fn browser_storage_clear(
        &self,
        Parameters(p): Parameters<StorageClearParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["storage".into(), p.r#type, "clear".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Tabs & Windows ──────────────────────────────────────────────────

    #[tool(description = "Create a new tab")]
    async fn browser_tab_new(
        &self,
        Parameters(p): Parameters<TabNewParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["tab".into(), "new".into()];
        if let Some(url) = p.url {
            a.push(url);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "List all open tabs")]
    async fn browser_tab_list(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["tab".into(), "list".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Switch to a tab by index")]
    async fn browser_tab_switch(
        &self,
        Parameters(p): Parameters<TabSwitchParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["tab".into(), p.index.to_string()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Close a tab by index (current tab if not specified)")]
    async fn browser_tab_close(
        &self,
        Parameters(p): Parameters<TabCloseParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["tab".into(), "close".into()];
        if let Some(idx) = p.index {
            a.push(idx.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Create a new window")]
    async fn browser_window_new(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["window".into(), "new".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Frames & Dialogs ────────────────────────────────────────────────

    #[tool(description = "Switch focus to an iframe by selector")]
    async fn browser_frame(
        &self,
        Parameters(p): Parameters<FrameParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["frame".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Switch focus back to the main frame")]
    async fn browser_frame_main(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["frame".into(), "main".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Accept a browser dialog (alert, confirm, prompt)")]
    async fn browser_dialog_accept(
        &self,
        Parameters(p): Parameters<DialogParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["dialog".into(), "accept".into()];
        if let Some(txt) = p.text {
            a.push(txt);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Dismiss a browser dialog")]
    async fn browser_dialog_dismiss(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["dialog".into(), "dismiss".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Debug & Diagnostics ─────────────────────────────────────────────

    #[tool(description = "Start recording a performance trace")]
    async fn browser_trace_start(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["trace".into(), "start".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Stop performance trace and save to file")]
    async fn browser_trace_stop(
        &self,
        Parameters(p): Parameters<TraceParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["trace".into(), "stop".into()];
        if let Some(ref path) = p.path {
            validated_path(path)?;
            a.push(path.clone());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Start Chrome DevTools profiling")]
    async fn browser_profiler_start(
        &self,
        Parameters(p): Parameters<ProfilerStartParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["profiler".into(), "start".into()];
        if let Some(cats) = p.categories {
            a.push("--categories".into());
            a.push(cats.join(","));
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Stop profiling and save to file")]
    async fn browser_profiler_stop(
        &self,
        Parameters(p): Parameters<TraceParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["profiler".into(), "stop".into()];
        if let Some(ref path) = p.path {
            validated_path(path)?;
            a.push(path.clone());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Start recording browser viewport to a video file (.webm)")]
    async fn browser_record_start(
        &self,
        Parameters(p): Parameters<RecordStartParams>,
    ) -> Result<CallToolResult, McpError> {
        validated_path(&p.path)?;
        let mut a = vec!["record".into(), "start".into(), p.path];
        if let Some(url) = p.url {
            a.push(url);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Stop video recording")]
    async fn browser_record_stop(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["record".into(), "stop".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Restart video recording with a new file")]
    async fn browser_record_restart(
        &self,
        Parameters(p): Parameters<RecordStartParams>,
    ) -> Result<CallToolResult, McpError> {
        validated_path(&p.path)?;
        let mut a = vec!["record".into(), "restart".into(), p.path];
        if let Some(url) = p.url {
            a.push(url);
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "View uncaught JavaScript exceptions on the page")]
    async fn browser_errors(
        &self,
        Parameters(p): Parameters<ConsoleParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["errors".into()];
        if p.clear.unwrap_or(false) {
            a.push("--clear".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Temporarily highlight an element for visual debugging")]
    async fn browser_highlight(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["highlight".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Open Chrome DevTools for the active page (requires headed mode)")]
    async fn browser_inspect(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["inspect".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Action Confirmation ─────────────────────────────────────────────

    #[tool(description = "Approve a sensitive action requiring manual confirmation")]
    async fn browser_confirm(
        &self,
        Parameters(p): Parameters<ConfirmationParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["confirm".into(), p.confirmation_id];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Deny a sensitive action requiring manual confirmation")]
    async fn browser_deny(
        &self,
        Parameters(p): Parameters<ConfirmationParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["deny".into(), p.confirmation_id];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── iOS Specific ────────────────────────────────────────────────────

    #[tool(description = "Tap on an element (alias for click, for iOS/touch)")]
    async fn browser_ios_tap(
        &self,
        Parameters(p): Parameters<SelectorParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["tap".into(), p.selector];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Perform a swipe gesture (iOS)")]
    async fn browser_ios_swipe(
        &self,
        Parameters(p): Parameters<SwipeParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["swipe".into(), p.direction];
        if let Some(dist) = p.distance {
            a.push(dist.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "List available iOS simulator devices")]
    async fn browser_ios_device_list(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["device".into(), "list".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Diffing ─────────────────────────────────────────────────────────

    #[tool(description = "Compare the current accessibility tree against the last snapshot or a baseline file")]
    async fn browser_diff_snapshot(
        &self,
        Parameters(p): Parameters<DiffSnapshotParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["diff".into(), "snapshot".into()];
        if let Some(ref b) = p.baseline {
            validated_path(b)?;
            a.push("--baseline".into());
            a.push(b.clone());
        }
        if let Some(sel) = p.selector {
            a.push("--selector".into());
            a.push(sel);
        }
        if p.compact.unwrap_or(false) {
            a.push("--compact".into());
        }
        if let Some(d) = p.depth {
            a.push("--depth".into());
            a.push(d.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Perform a visual pixel diff against a baseline screenshot")]
    async fn browser_diff_screenshot(
        &self,
        Parameters(p): Parameters<DiffScreenshotParams>,
    ) -> Result<CallToolResult, McpError> {
        validated_path(&p.baseline)?;
        let mut a = vec!["diff".into(), "screenshot".into(), "--baseline".into(), p.baseline];
        if let Some(ref o) = p.output {
            validated_path(o)?;
            a.push("--output".into());
            a.push(o.clone());
        }
        if let Some(t) = p.threshold {
            a.push("--threshold".into());
            a.push(t.to_string());
        }
        if let Some(sel) = p.selector {
            a.push("--selector".into());
            a.push(sel);
        }
        if p.full_page.unwrap_or(false) {
            a.push("--full".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Compare two URLs (snapshot diff and optionally visual diff)")]
    async fn browser_diff_url(
        &self,
        Parameters(p): Parameters<DiffUrlParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["diff".into(), "url".into(), p.url1, p.url2];
        if p.screenshot.unwrap_or(false) {
            a.push("--screenshot".into());
        }
        if p.full_page.unwrap_or(false) {
            a.push("--full".into());
        }
        if let Some(wu) = p.wait_until {
            a.push("--wait-until".into());
            a.push(wu);
        }
        if let Some(sel) = p.selector {
            a.push("--selector".into());
            a.push(sel);
        }
        if p.compact.unwrap_or(false) {
            a.push("--compact".into());
        }
        if let Some(d) = p.depth {
            a.push("--depth".into());
            a.push(d.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    // ── Batch Execution ─────────────────────────────────────────────────

    #[tool(description = "Execute multiple commands in a single CLI invocation (avoids process startup overhead)")]
    async fn browser_batch(
        &self,
        Parameters(p): Parameters<BatchParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["batch".into(), "--json".into()];
        if p.bail.unwrap_or(false) {
            a.push("--bail".into());
        }
        a.extend(session_args(p.session_id.as_deref())?);

        let stdin = serde_json::to_string(&p.commands).map_err(|e| {
            McpError::invalid_params(format!("Failed to serialize batch commands: {e}"), None)
        })?;

        run_with_stdin(a, stdin).await
    }

    // ── Streaming ───────────────────────────────────────────────────────

    #[tool(description = "Start runtime WebSocket streaming of the browser viewport")]
    async fn browser_stream_enable(
        &self,
        Parameters(p): Parameters<StreamEnableParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["stream".into(), "enable".into()];
        if let Some(port) = p.port {
            a.push("--port".into());
            a.push(port.to_string());
        }
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Show runtime streaming state and bound port")]
    async fn browser_stream_status(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["stream".into(), "status".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }

    #[tool(description = "Stop runtime WebSocket streaming")]
    async fn browser_stream_disable(
        &self,
        Parameters(p): Parameters<SessionOnly>,
    ) -> Result<CallToolResult, McpError> {
        let mut a = vec!["stream".into(), "disable".into()];
        a.extend(session_args(p.session_id.as_deref())?);
        run(a).await
    }
}

#[tool_handler]
impl ServerHandler for AgentBrowserServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "agent-browser",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Browser automation MCP server. Wraps Vercel's agent-browser CLI to provide \
                 navigation, interaction, screenshots, session management, and more. \
                 Requires agent-browser to be installed: cargo install agent-browser && agent-browser install",
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_args() {
        assert_eq!(session_args(None).unwrap(), Vec::<String>::new());
        assert_eq!(
            session_args(Some("my-session")).unwrap(),
            vec!["--session", "my-session"]
        );
        assert!(session_args(Some("invalid session!")).is_err());
    }

    #[test]
    fn test_path_validation() {
        assert!(validated_path("safe.png").is_ok());
        assert!(validated_path("./safe/path.txt").is_ok());
        assert!(validated_path("/etc/passwd").is_err());
        assert!(validated_path("../traversal").is_err());
        assert!(validated_path("dir/../../traversal").is_err());
    }

    #[test]
    fn test_session_id_validation() {
        assert!(validate_session_id("my-session-123").is_ok());
        assert!(validate_session_id("session.name").is_ok());
        assert!(validate_session_id("session_name").is_ok());
        assert!(validate_session_id("invalid!").is_err());
        assert!(validate_session_id("").is_err());
    }
}
