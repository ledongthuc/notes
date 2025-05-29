from agno.agent import Agent
from agno.models.openai import OpenAIChat
from agno.tools.duckduckgo import DuckDuckGoTools

agent = Agent(
    model=OpenAIChat(id="gpt-4o-mini"),
    description="You are a digital marketing designer. You are responsible for creating a poster design ideas. You will use the internet to search for the latest trends and ideas.",
    tools=[DuckDuckGoTools()],
    show_tool_calls=True,
    # markdown=True
)
agent.print_response("Create a poster design ideas for a new product banking home loan target audience is 22-27 year old.", stream=True)