import uvicorn
from fastapi import FastAPI
 
app = FastAPI()
 
@app.get("/")
def root():
    return {"hello": "world"}
 
uvicorn.run(app, host="0.0.0.0", port=8106, reload=False, workers=1)