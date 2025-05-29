import os

from haystack import Pipeline
from haystack.components.fetchers import LinkContentFetcher
from haystack.components.converters import HTMLToDocument
from haystack.components.builders import ChatPromptBuilder
from haystack.components.generators.chat import OpenAIChatGenerator
from haystack.dataclasses import ChatMessage

fetcher = LinkContentFetcher()
converter = HTMLToDocument()
prompt_template = [
    ChatMessage.from_user(
      """
      According to the contents of this website:
      {% for document in documents %}
        {{document.content}}
      {% endfor %}
      Answer the given question: {{query}}
      Answer:
      """
    )
]

prompt_builder = ChatPromptBuilder(template=prompt_template)
llm = OpenAIChatGenerator()

pipeline = Pipeline()
pipeline.add_component("fetcher", fetcher)
pipeline.add_component("converter", converter)
pipeline.add_component("prompt", prompt_builder)
pipeline.add_component("llm", llm)

pipeline.connect("fetcher.streams", "converter.sources")
pipeline.connect("converter.documents", "prompt.documents")
pipeline.connect("prompt.prompt", "llm")

result = pipeline.run({"fetcher": {"urls": ["https://thuc.space/posts/rust-vector-capacity-increment/"]},
              "prompt": {"query": "Which components do I need for a RAG pipeline?"}})

print(result["llm"]["replies"][0].text)
