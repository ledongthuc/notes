from fastapi import FastAPI
import uvicorn
from llama_index.core.agent.workflow import FunctionAgent
from llama_index.llms.openai import OpenAI

# Create FastAPI app instance
app = FastAPI(
    title="Test FastAPI App",
    description="A simple FastAPI application example",
    version="1.0.0"
)

agent = FunctionAgent(
    llm=OpenAI(model="gpt-4o-mini"),
    system_prompt="""You are personal trainer to help build workout flow.""",
)


@app.get("/")
async def root():
    """Root endpoint"""
    return {"message": "Hello from test-with-llamaindex-fastapi2!", "status": "success"}

@app.get("/items/{item_id}")
async def read_item(item_id: int, q: str | None = None):
    """Get item by ID with optional query parameter"""
    return {"item_id": item_id, "q": q}\

@app.get("/demo-llm-index")
async def chat():
    """Chat endpoint"""
    response = await agent.run("How many workout day should I do a week for grain muscle?")
    print(response)
    return {"message": response}

@app.post("/items/")
async def create_item(name: str, price: float):
    """Create a new item"""
    return {"name": name, "price": price, "created": True}

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy", "service": "test-with-llamaindex-fastapi2"}

def main():
    """Run the FastAPI application with uvicorn"""
    print("Starting FastAPI server with uvicorn...")
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,  # Enable auto-reload for development
        log_level="info"
    )

if __name__ == "__main__":
    main()
