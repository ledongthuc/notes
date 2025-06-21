# uvicorn server_interating_fastapi:app --port 8086 --reload
from fastmcp import FastMCP
from fastapi import FastAPI

# Constants
mcp = FastMCP("todo-list")
mcp_app = mcp.http_app()
app = FastAPI(
    lifespan=mcp_app.router.lifespan_context,
)
app.mount("/mcp-server", mcp_app, "mcp")
todo_items: list[str] = []


@mcp.resource("resource://todo-list")
async def list_todo_resource() -> list[str]:
    """Get all todo options

    Args:
        None
    """
    return todo_items


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
