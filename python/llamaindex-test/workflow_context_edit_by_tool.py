from llama_index.core.agent.workflow import AgentWorkflow
from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import Context
import asyncio


async def set_name(ctx: Context, name: str) -> str:
    state = await ctx.get("state")
    state["name"] = name
    await ctx.set("state", state)
    return f"Name set to {name}"


agent = AgentWorkflow.from_tools_or_functions(
    [set_name],
    llm=OpenAI(model="gpt-4o-mini"),
    system_prompt="""You are a helpful assistant that can set a name.""",
    initial_state={"name": "unset"},
)


async def main():
    ctx = Context(agent)

    response = await agent.run("What's my name?", ctx=ctx)
    print(response)

    response = await agent.run("My name is Thuc", ctx=ctx)
    print(response)

    response = await agent.run("What's my name?", ctx=ctx)
    print(response)


# Run the agent
if __name__ == "__main__":
    asyncio.run(main())
