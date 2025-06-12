import asyncio
from fastmcp import Client, FastMCP

server_instance = FastMCP(name="todo-list")
http_url = "http://127.0.0.1:8000/mcp"

client_http = Client(http_url)


print(client_http.transport)

async def call_tool():
    async with client_http:
        tools = await client_http.list_tools()
        print(tools)
        result = await client_http.call_tool("add_todo", {"item": "test"})
        print(result)

asyncio.run(call_tool())