from llama_index.llms.openai import OpenAI

handle = OpenAI().stream_complete("Vietnam is ")

for token in handle:
    print(token.delta, end="", flush=True)
