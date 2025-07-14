from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import (
    StartEvent,
    StopEvent,
    Workflow,
    step,
    Event,
    Context,
)


class FirstEvent(Event):
    first_output: str


class SecondEvent(Event):
    second_output: str
    response: str


class ProgressEvent(Event):
    msg: str


class MyWorkflow(Workflow):
    @step
    async def step_one(self, ctx: Context, ev: StartEvent) -> FirstEvent:
        ctx.write_event_to_stream(ProgressEvent(msg="Step one is happening"))
        return FirstEvent(first_output="First step complete.")

    @step
    async def step_two(self, ctx: Context, ev: FirstEvent) -> SecondEvent | FirstEvent:
        llm = OpenAI(model="gpt-4o-mini")
        generator = await llm.astream_complete(
            "Please give me a random short lyric songs in 2 sentences."
        )
        async for response in generator:
            ctx.write_event_to_stream(ProgressEvent(msg=response.delta))

        return SecondEvent(
            second_output="Second step complete, full response attached",
            response=str(response),
        )

    @step
    async def step_three(self, ctx: Context, ev: SecondEvent) -> StopEvent:
        ctx.write_event_to_stream(ProgressEvent(msg="Step three is happening"))
        return StopEvent(result="Workflow complete.")


async def main():
    w = MyWorkflow(timeout=30, verbose=True)
    handler = w.run(first_input="Start the workflow.")

    async for ev in handler.stream_events():
        if isinstance(ev, ProgressEvent):
            print(ev.msg)

    final_result = await handler
    print("Final result", final_result)


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())


# draw_all_possible_flows(MyWorkflow, filename="basic_workflow.html")
