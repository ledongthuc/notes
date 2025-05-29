import asyncio
import json
import os
from pathlib import Path
from typing import Optional
from contextlib import AsyncExitStack

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

from openai import OpenAI


from dotenv import load_dotenv

load_dotenv()


class MCPClient:
    def __init__(self):
        self.session: Optional[ClientSession] = None
        self.exit_stack = AsyncExitStack()
        self.client = OpenAI(
            api_key=os.environ.get("OPENAI_API_KEY"),
        )
        self.model = "gpt-4o-2024-08-06"

    async def connect_to_server(self):
        """Connect to an MCP server"""
        server_script_path = str(Path("server.py").resolve())

        server_params = StdioServerParameters(
            command="python",
            args=[server_script_path],
            env=None
        )

        stdio_transport = await self.exit_stack.enter_async_context(stdio_client(server_params))
        self.stdio, self.write = stdio_transport
        self.session = await self.exit_stack.enter_async_context(ClientSession(self.stdio, self.write))

        await self.session.initialize()

        # List available tools
        response = await self.session.list_tools()
        tools = response.tools
        print("\nConnected to server with tools:", [tool.name for tool in tools])

    async def process_query(self, query: str) -> str:
        """Process a query using Claude and available tools"""
        messages = [
            {
                "role": "user",
                "content": query
            }
        ]

        response = await self.session.list_tools()
        available_tools = [{
            "name": tool.name,
            "description": tool.description,
            "input_schema": tool.inputSchema,
            "type": "function",
        } for tool in response.tools]
        
        response = self.client.responses.create(
            model=self.model,
            instructions="You are a weather agent. You are responsible for providing weather information.",
            input=messages,
            tools=available_tools,
        )
        # print(response)

        # Response(id='resp_683830518dbc81989880c3721517fc1b0df073c8f35c8005', created_at=1748512849.0, error=None, incomplete_details=None, instructions='You are a weather agent. You are responsible for providing weather information.', metadata={}, model='gpt-4o-2024-08-06', object='response', output=[ResponseFunctionToolCall(arguments='{"latitude":35.6762,"longitude":139.6503}', call_id='call_WfhgRm9rrcDWVZHSRAi0cpNV', name='get_forecast', type='function_call', id='fc_68383052425081988290f36fb408bf130df073c8f35c8005', status='completed')], parallel_tool_calls=True, temperature=1.0, tool_choice='auto', tools=[FunctionTool(name='get_alerts', parameters={'type': 'object', 'properties': {}}, strict=True, type='function', description='Get weather alerts for a US state.\n\nArgs:\n    state: Two-letter US state code (e.g. CA, NY)\n'), FunctionTool(name='get_forecast', parameters={'type': 'object', 'properties': {}}, strict=True, type='function', description='Get weather forecast for a location.\n\nArgs:\n    latitude: Latitude of the location\n    longitude: Longitude of the location\n')], top_p=1.0, background=False, max_output_tokens=None, previous_response_id=None, reasoning=Reasoning(effort=None, generate_summary=None, summary=None), service_tier='default', status='completed', text=ResponseTextConfig(format=ResponseFormatText(type='text')), truncation='disabled', usage=ResponseUsage(input_tokens=109, input_tokens_details=InputTokensDetails(cached_tokens=0), output_tokens=26, output_tokens_details=OutputTokensDetails(reasoning_tokens=0), total_tokens=135), user=None, store=True)

        # Process response and handle tool calls

        for content in response.output:
            if content.type == 'function_call':
                tool_name = content.name
                print(content.arguments)
                tool_args = json.loads(content.arguments)

                # Execute tool call
                result = await self.session.call_tool(tool_name, tool_args)

                messages.append({
                    "type": "function_call",
                    "name": content.name,
                    "arguments": content.arguments,
                    "id": content.id,
                    "call_id": content.call_id
                })
                messages.append({
                    "type": "function_call_output",
                    "output": result.content[0].text,
                    "call_id": content.call_id,
                })
        
        response = self.client.responses.create(
            model=self.model,
            instructions="You are a weather agent. You are responsible for providing weather information.",
            input=messages,
            tools=available_tools,
        )

        print(response)
        return response.output[0].content[0].text

    async def cleanup(self):
        """Clean up resources"""
        await self.exit_stack.aclose()

async def main():
    client = MCPClient()
    try:
        await client.connect_to_server()
        response = await client.process_query("What’s the weather in Sacramento?")
        print(response)
    finally:
        await client.cleanup()

if __name__ == "__main__":
    asyncio.run(main())