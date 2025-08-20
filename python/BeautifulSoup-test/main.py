import requests
from bs4 import BeautifulSoup


def read_page_text(url):
    res = requests.get(url, timeout=10)
    res.raise_for_status()

    soup = BeautifulSoup(res.text, "html.parser")

    # remove script & style
    for tag in soup(["script", "style"]):
        tag.decompose()

    # get text
    text = soup.get_text(separator=" ", strip=True)
    return text


if __name__ == "__main__":
    url = "https://coinmarketcap.com/community/articles/68a484773a09ab7ecc32c4ba/"
    print(read_page_text(url))
