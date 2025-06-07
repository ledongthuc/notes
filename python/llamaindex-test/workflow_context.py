from llama_index.core.agent.workflow import FunctionAgent
from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import Context, JsonSerializer
import asyncio
import json

agent = FunctionAgent(
    llm=OpenAI(model="gpt-4o-mini"),
    system_prompt="""You are personal trainer to help build workout flow.""",
)


async def main():
    ctx = Context(agent)

    response = await agent.run(
        "How many workout day should I do a week for grain muscle?", ctx=ctx
    )
    print(response)
    ctx_dict = ctx.to_dict(serializer=JsonSerializer())
    context_dump = json.dumps(ctx_dict)

    ctx_dict = json.loads(context_dump)
    ctx = Context.from_dict(agent, ctx_dict, serializer=JsonSerializer())
    response = await agent.run(
        "Give me quick short list of workout plan for that days", ctx=ctx
    )
    print(response)
    ctx_dict = ctx.to_dict(serializer=JsonSerializer())
    context_dump = json.dumps(ctx_dict)


# Run the agent
if __name__ == "__main__":
    asyncio.run(main())
