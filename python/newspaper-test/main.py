from newspaper import Article


def read_article(url):
    article = Article(url, language="en")
    article.download()
    article.parse()
    return {"title": article.title, "text": article.text}


if __name__ == "__main__":
    url = "https://coinmarketcap.com/community/articles/68a484773a09ab7ecc32c4ba/"
    result = read_article(url)
    print("Title:", result["title"])
    print("Content:\n", result["text"])
