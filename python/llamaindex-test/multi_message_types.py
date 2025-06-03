from llama_index.llms.openai import OpenAI
from llama_index.core.llms import ChatMessage, TextBlock, ImageBlock

messages = [
    ChatMessage(
        role="user",
        blocks=[
            ImageBlock(path="data/crowd.jpg"),
            TextBlock(text="How many people in the pictures. List all their emotions"),
        ],
    )
]

handle = OpenAI(model="gpt-4o-mini").chat(messages)

print(handle)
