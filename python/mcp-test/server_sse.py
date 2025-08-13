"""
Sample MCP Server for ChatGPT Integration

This server implements the Model Context Protocol (MCP) with search and fetch
capabilities designed to work with ChatGPT's chat and deep research features.
"""

import logging

from fastmcp import FastMCP

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

todo_items: list[str] = []

server_instructions = """
This MCP server provides search and document retrieval capabilities
for chat and deep research connectors. Use the search tool to find relevant documents
based on keywords, then use the fetch tool to retrieve complete
document content with citations.
"""


def create_server():
    """Create and configure the MCP server with search and fetch tools."""

    # Initialize the FastMCP server
    mcp = FastMCP(name="Sample MCP Server",
                  instructions=server_instructions)

    @mcp.tool()
    async def list_todo() -> list[str]:
        """Get all todo list items

        Args:
            None
        """
        return todo_items


    @mcp.tool()
    async def add_todo(item: str) -> list[str]:
        """Add a todo item

        Args:
            item: The todo item to add
        """
        todo_items.append(item)
        return todo_items


    @mcp.tool()
    async def remove_todo(item: str) -> list[str]:
        """Remove a todo item

        Args:
            item: The todo item to remove
        """
        todo_items.remove(item)
        return todo_items

    return mcp


def main():
    """Main function to start the MCP server."""
    # Create the MCP server
    server = create_server()

    # Configure and start the server
    logger.info("Starting MCP server on 0.0.0.0:8000")
    logger.info("Server will be accessible via SSE transport")

    try:
        # Use FastMCP's built-in run method with SSE transport
        server.run(transport="sse", host="0.0.0.0", port=8000)
    except KeyboardInterrupt:
        logger.info("Server stopped by user")
    except Exception as e:
        logger.error(f"Server error: {e}")
        raise


if __name__ == "__main__":
    main()