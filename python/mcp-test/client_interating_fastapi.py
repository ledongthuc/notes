import asyncio

from fastmcp import Client


async def main():
    async with Client("http://127.0.0.1:8086/mcp-server/mcp") as client:
        result = await client.call_tool("add_todo", {"item": "New item"})
        print(result)


if __name__ == "__main__":
    asyncio.run(main())
