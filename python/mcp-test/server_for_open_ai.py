
import logging
from typing import Dict, List, Any

from fastmcp import FastMCP

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

server_instructions = """
This MCP server provides information about advertising Home Loan, banking, insurance, and other products in the store.
"""

store_items = [
            {"id": "1", "title": "Iphone X max", "text": "The latest iPhone with a stunning 6.7-inch Super Retina XDR display and powerful A14 Bionic chip.", "url": "https://personalisationhub.com/", "metadata": {"category": "electronics"}},
            {"id": "2", "title": "Samsung S23", "text": "Experience next-level photography and performance with the Samsung Galaxy S23's advanced camera system and Snapdragon processor.", "url": "https://personalisationhub.com/", "metadata": {"category": "electronics"}},
            {"id": "3", "title": "Home loan", "text": "Flexible home loan options with competitive interest rates and personalized repayment plans to help you achieve your dream home.", "url": "https://personalisationhub.com/", "metadata": {"category": "financial_services"}},
            {"id": "4", "title": "Nike Air Max", "text": "Classic athletic shoes featuring Nike's revolutionary Air cushioning technology for maximum comfort and style.", "url": "https://personalisationhub.com/", "metadata": {"category": "fashion"}},
            {"id": "5", "title": "PlayStation 5", "text": "Next-generation gaming console with lightning-fast loading, stunning 4K graphics, and innovative DualSense controller.", "url": "https://personalisationhub.com/", "metadata": {"category": "electronics"}},
            {"id": "6", "title": "Credit Card", "text": "Premium credit card with cashback rewards, travel benefits, and zero annual fees for the first year.", "url": "https://personalisationhub.com/", "metadata": {"category": "financial_services"}},
            {"id": "7", "title": "MacBook Pro", "text": "Professional-grade laptop with M2 chip, stunning Retina display, and all-day battery life.", "url": "https://personalisationhub.com/", "metadata": {"category": "electronics"}},
            {"id": "8", "title": "Car Insurance", "text": "Comprehensive auto insurance coverage with competitive rates and 24/7 roadside assistance.", "url": "https://personalisationhub.com/", "metadata": {"category": "insurance"}},
            {"id": "9", "title": "Adidas Ultraboost", "text": "Premium running shoes with responsive Boost cushioning and breathable Primeknit upper.", "url": "https://personalisationhub.com/", "metadata": {"category": "fashion"}},
            {"id": "10", "title": "Investment Fund", "text": "Diversified investment portfolio managed by expert financial advisors with proven track record.", "url": "https://personalisationhub.com/", "metadata": {"category": "financial_services"}}
        ]


def create_server():
    mcp = FastMCP(name="Sample MCP Server",
                  instructions=server_instructions)

    @mcp.tool()
    async def search(query: str) -> Dict[str, List[Dict[str, Any]]]:        
        logger.info(f"Searching for {query}")
        # Split query into keywords
        keywords = query.lower().split()
        
        # Count matches for each item
        item_matches = []
        for item in store_items:
            matches = 0
            title_text = (item["title"] + " " + item["text"]).lower()
            for keyword in keywords:
                if keyword in title_text:
                    matches += 1
            if matches > 0:
                item_matches.append((matches, item))
        
        # Sort by number of matches (descending) and take top 5
        item_matches.sort(key=lambda x: x[0], reverse=True)
        results = [item for matches, item in item_matches[:5]]
        
        return {"results": results}

    @mcp.tool()
    async def fetch(id: str) -> Dict[str, Any]:
        logger.info(f"Fetching item with id {id}")
        for item in store_items:
            if item["id"] == id:
                return item
        raise ValueError(f"Item with id {id} not found")

    return mcp


def main():
    server = create_server()

    logger.info("Starting MCP server on 0.0.0.0:8000")
    logger.info("Server will be accessible via SSE transport")

    try:
        server.run(transport="sse", host="0.0.0.0", port=8000, path="/mcp")
    except KeyboardInterrupt:
        logger.info("Server stopped by user")
    except Exception as e:
        logger.error(f"Server error: {e}")
        raise


if __name__ == "__main__":
    main()