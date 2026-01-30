use std::sync::Arc;
use tokio::sync::Mutex;

use rmcp::{
    ErrorData, ServerHandler,
    handler::server::tool::ToolRouter,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool, tool_handler, tool_router,
    transport::stdio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = HelloWorld::new().serve(stdio()).await.inspect_err(|e| {
        println!("Error starting HelloWorld service: {}", e);
    })?;
    service.waiting().await?;
    Ok(())
}

#[derive(Clone)]
pub struct HelloWorld {
    counter: Arc<Mutex<i32>>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl HelloWorld {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Increments by 1 and returns the current count")]
    async fn increment(&self) -> Result<CallToolResult, ErrorData> {
        let mut count = self.counter.lock().await;
        *count += 1;
        Ok(CallToolResult::success(vec![Content::text(
            count.to_string(),
        )]))
    }

    #[tool(description = "Decrements by 1 and returns the current count")]
    async fn decrement(&self) -> Result<CallToolResult, ErrorData> {
        let mut count = self.counter.lock().await;
        *count -= 1;
        Ok(CallToolResult::success(vec![Content::text(
            count.to_string(),
        )]))
    }

    #[tool(description = "Returns the current count")]
    async fn get_value(&self) -> Result<CallToolResult, ErrorData> {
        let count = self.counter.lock().await;
        Ok(CallToolResult::success(vec![Content::text(
            (*count).to_string(),
        )]))
    }

    #[tool(description = "Hello, from your first Rust MCP server!")]
    fn echo(&self) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::success(vec![Content::text(
            "Hello, from your first Rust MCP server!".to_string(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for HelloWorld {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(String::from(
                "Use the provided tools to interact with the HelloWorld server.",
            )),
        }
    }
}
