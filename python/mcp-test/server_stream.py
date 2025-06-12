from fastmcp import FastMCP

# Constants
mcp = FastMCP("todo-list")
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


if __name__ == "__main__":
    print("Starting todo list MCP server...")
    # Initialize and run the server
    mcp.run(transport="streamable-http", host="127.0.0.1", port=8000, path="/mcp")
