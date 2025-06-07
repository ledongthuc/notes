from llama_index.llms.openai import OpenAI
from llama_index.core.agent.workflow import AgentWorkflow
from llama_index.core.workflow import Context, InputRequiredEvent, HumanResponseEvent
import asyncio


# a tool that performs a dangerous task
async def dangerous_task(ctx: Context) -> str:
    """A dangerous task that requires human confirmation."""

    # emit an event to the external stream to be captured
    ctx.write_event_to_stream(
        InputRequiredEvent(
            prefix="Are you sure you want to proceed? ",
        )
    )

    # wait until we see a HumanResponseEvent
    response = await ctx.wait_for_event(HumanResponseEvent)

    # act on the input from the event
    if response.response.strip().lower() == "yes":
        return "Dangerous task completed successfully."
    else:
        return "Dangerous task aborted."


workflow = AgentWorkflow.from_tools_or_functions(
    [dangerous_task],
    llm=OpenAI(model="gpt-4o-mini"),
    system_prompt="You are a helpful assistant that can perform dangerous tasks.",
)


async def main():
    handler = workflow.run(user_msg="I want to proceed with the dangerous task.")

    async for event in handler.stream_events():
        # capture InputRequiredEvent
        if isinstance(event, InputRequiredEvent):
            # capture keyboard input
            response = input(event.prefix)
            # send our response back
            handler.ctx.send_event(
                HumanResponseEvent(
                    response=response,
                )
            )

    response = await handler
    print(str(response))


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())
