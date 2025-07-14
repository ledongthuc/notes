from llama_index.core import download_loader
import os

from llama_index.readers.database import DatabaseReader
from llama_index.core import VectorStoreIndex


reader = DatabaseReader(
    scheme=os.getenv("DB_SCHEME"),
    host=os.getenv("DB_HOST"),
    port=os.getenv("DB_PORT"),
    user=os.getenv("DB_USER"),
    password=os.getenv("DB_PASS"),
    dbname=os.getenv("DB_NAME"),
)

query = "SELECT * FROM user_data.users"
documents = reader.load_data(query=query)

vector_index = VectorStoreIndex.from_documents(documents)
vector_index.as_query_engine()
