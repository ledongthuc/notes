from llama_index.llms.openai import OpenAI
from llama_index.tools.duckduckgo import DuckDuckGoSearchToolSpec
from llama_index.core.agent.workflow import FunctionAgent, AgentStream
import asyncio

# Directly from open AI
handle = OpenAI().stream_complete("Ho Chi Minh city is ")

for token in handle:
    print(token.delta, end="", flush=True)
print("\n==============\n")

# Stream workflow
workflow = FunctionAgent(
    tools=DuckDuckGoSearchToolSpec().to_tool_list(),
    llm=OpenAI(model="gpt-4o-mini"),
    system_prompt="""You're a helpful assistant that can search the web for information.""",
)


async def main():
    handler = workflow.run(user_msg="Any news from searching in Ho Chi Minh city?")
    async for event in handler.stream_events():
        if isinstance(event, AgentStream):
            print(event.delta, end="", flush=True)


# Run the agent
if __name__ == "__main__":
    asyncio.run(main())
