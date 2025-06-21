import asyncio
from typing import Callable, Any
from mcp.server.fastmcp import FastMCP

# Constants
mcp = FastMCP("Do action by number")


def make_callable(number: int) -> Callable[..., Any]:
    def dynamic_function(*args, **kwargs):
        print("Arguments: {", number, "}: ", args, kwargs)
        return number

    return dynamic_function


async def add_tool_in_background():
    i = 0
    while True:
        await asyncio.sleep(2)
        i = i + 1
        dynamic_callable = make_callable(i)
        mcp.add_tool(
            fn=dynamic_callable,
            name="Do action number " + str(i),
            description="Do action number " + str(i),
        )


async def main():
    results = await asyncio.gather(
        mcp.run_stdio_async(),
        add_tool_in_background(),
    )
    print(results)


if __name__ == "__main__":
    asyncio.run(main())
