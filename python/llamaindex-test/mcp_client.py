from llama_index.tools.mcp import (
    aget_tools_from_mcp_url,
)
from llama_index.core.agent.workflow import FunctionAgent
from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import Context
import asyncio


async def main():
    tools = await aget_tools_from_mcp_url("http://127.0.0.1:8000/mcp")

    agent = FunctionAgent(
        tools=tools,
        llm=OpenAI(model="gpt-4o-mini"),
        system_prompt="""You are a helpful assistant that can use tool to list, add and remove todo items from user free text input.""",
    )

    ctx = Context(agent)

    response = await agent.run("List all todo items. Say nothing if there are no items.", ctx=ctx)
    print(response)

    response = await agent.run("Add a new todo item: 'Buy groceries'", ctx=ctx)
    print(response)

    response = await agent.run("Add the todo item: 'Sell house'", ctx=ctx)
    print(response)

    response = await agent.run("List all todo items. Say nothing if there are no items.", ctx=ctx)
    print(response)

    response = await agent.run("Remove the todo item: 'Buy groceries'", ctx=ctx)
    print(response)

    response = await agent.run("List all todo items. Say nothing if there are no items.", ctx=ctx)
    print(response)


# Run the agent
if __name__ == "__main__":
    asyncio.run(main())
