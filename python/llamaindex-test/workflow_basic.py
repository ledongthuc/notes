from llama_index.llms.openai import OpenAI
from llama_index.core.workflow import (
    StartEvent,
    StopEvent,
    Workflow,
    step,
    Event,
)
from llama_index.utils.workflow import draw_all_possible_flows


class FirstEvent(Event):
    first_output: str
    count: int


class SecondEvent(Event):
    second_output: str


class MyWorkflow(Workflow):
    @step
    async def step_one(self, ev: StartEvent) -> FirstEvent:
        print(ev.first_input)
        return FirstEvent(first_output="First step complete.", count=1)

    @step
    async def step_two(self, ev: FirstEvent) -> SecondEvent | FirstEvent:
        print(ev.first_output, "Count: ", ev.count)
        if ev.count < 5:
            return FirstEvent(first_output="First step complete.", count=ev.count + 1)
        return SecondEvent(second_output="Second step complete.")

    @step
    async def step_three(self, ev: SecondEvent) -> StopEvent:
        print(ev.second_output)
        return StopEvent(result="Workflow complete.")


async def main():
    w = MyWorkflow(timeout=10, verbose=False)
    result = await w.run(first_input="Start the workflow.")
    print(result)


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())


# draw_all_possible_flows(MyWorkflow, filename="basic_workflow.html")
