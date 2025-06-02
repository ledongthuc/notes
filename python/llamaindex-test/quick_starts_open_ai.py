from llama_index.core import VectorStoreIndex, SimpleDirectoryReader
from llama_index.core.agent.workflow import FunctionAgent
from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import Context
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


async def search_documents(query: str) -> str:
    """Useful for answering natural language questions about an personal"""
    response = await query_engine.aquery(query)
    return str(response)


agent = FunctionAgent(
    tools=[multiply, search_documents],
    llm=OpenAI(model="gpt-4o-mini"),
    system_prompt="""You are a helpful assistant that can perform calculations
    and search through documents to answer questions.""",
)


async def main():
    ctx = Context(agent)

    response = await agent.run("How many languages they speeks", ctx=ctx)
    print(response)

    response = await agent.run(
        "Multiple number of languages they speeks with 2. Show the result", ctx=ctx
    )
    print(response)


# Run the agent
if __name__ == "__main__":
    asyncio.run(main())
