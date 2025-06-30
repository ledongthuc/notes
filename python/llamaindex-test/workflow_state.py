from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import (
    StartEvent,
    StopEvent,
    Workflow,
    step,
    Event,
    Context,
)
from llama_index.utils.workflow import draw_all_possible_flows


class FirstEvent(Event):
    first_output: str
    count: int


class SecondEvent(Event):
    second_output: str


class MyWorkflow(Workflow):
    @step
    async def step_one(self, ctx: Context, ev: StartEvent) -> FirstEvent:
        print(ev.first_input)
        await ctx.set("counting_state", 1)
        return FirstEvent(first_output="First step complete.", count=1)

    @step
    async def step_two(self, ctx: Context, ev: FirstEvent) -> SecondEvent | FirstEvent:
        print(ev.first_output, "Count: ", ev.count)
        counting_stage = await ctx.get("counting_state")
        counting_stage = counting_stage + 1
        await ctx.set("counting_state", counting_stage)
        print("Counting stage: ", counting_stage)
        if ev.count < 5:
            return FirstEvent(first_output="First step complete.", count=ev.count + 1)
        return SecondEvent(second_output="Second step complete.")

    @step
    async def step_three(self, ctx: Context, ev: SecondEvent) -> StopEvent:
        print(ev.second_output)
        counting_stage = await ctx.get("counting_state")
        counting_stage + 1
        await ctx.set("counting_state", counting_stage)
        print("Counting stage: ", counting_stage)
        return StopEvent(result="Workflow complete.")


async def main():
    w = MyWorkflow(timeout=10, verbose=False)
    result = await w.run(first_input="Start the workflow.")
    print(result)


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())


# draw_all_possible_flows(MyWorkflow, filename="basic_workflow.html")
