from llama_index.llms.openai import OpenAI
from llama_index.core.prompts import ChatMessage

messages = [
    ChatMessage(role="system", content="You are a helpful assistant."),
    ChatMessage(role="user", content="Vietnam is "),
]

handle = OpenAI().chat(messages)

print(handle)
