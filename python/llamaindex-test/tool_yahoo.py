from llama_index.core import VectorStoreIndex, SimpleDirectoryReader
from llama_index.core.agent.workflow import FunctionAgent
from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import Context
from llama_index.tools.yahoo_finance import YahooFinanceToolSpec
import asyncio

documents = SimpleDirectoryReader("data").load_data()
index = VectorStoreIndex.from_documents(documents)

# Store index to storage file
# index.storage_context.persist("storage")

# Load index from storage file
# storage_context = StorageContext.from_defaults(persist_dir="storage")
# index = load_index_from_storage(storage_context)

query_engine = index.as_query_engine()


def multiply(a: float, b: float) -> float:
    """Useful for multiplying two numbers."""
    return a * b


def add(a: float, b: float) -> float:
    """Useful for adding two numbers."""
    return a + b


finance_tools = YahooFinanceToolSpec().to_tool_list()
finance_tools.extend([multiply, add])

agent = FunctionAgent(
    tools=finance_tools,
    llm=OpenAI(model="gpt-4o-mini"),
    system_prompt="""You are a helpful assistant that can perform calculations
    and search through documents to answer questions.""",
)


async def main():
    ctx = Context(agent)

    response = await agent.run("What's the current stock price of NVIDIA?", ctx=ctx)
    print(response)

    response = await agent.run("What's the current stock price of Facebook?", ctx=ctx)
    print(response)

    response = await agent.run("Sum of NVIDIA and Facebook", ctx=ctx)
    print(response)


# Run the agent
if __name__ == "__main__":
    asyncio.run(main())
