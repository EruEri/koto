from PIL import Image
import requests

def show_image(url : str, artist : str = "") -> None:
    print("called ... ")
    im = Image.open(requests.get(url, stream=True).raw)
    im.show(artist)



