from agno.agent import Agent
from agno.models.openai import OpenAIChat

agent = Agent(
    model=OpenAIChat(id="gpt-4o-mini"),
    description="You are a digital marketing designer. You are responsible for creating a landing page design ideas.",
    # markdown=True
)
agent.print_response("Create a landing page design ideas for a new product banking home loan.", stream=True)