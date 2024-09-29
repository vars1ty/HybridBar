import requests

def getData():
    data = requests.get("https://wakatime.com/api/v1/users/RyannKim327/status_bar/today")
    # data = requests.get("https://wakatime.com/api/v1/leaders")
    print(data.json())
    # return requests.get("https://gist.githubusercontent.com/RyannKim327/e99f1a5e9f777937f55d4936ab471bae/raw/e95fc63c479e051b0a55d49c94257e42ee1c4a16/%25F0%259F%2593%258A%2520Weekly%2520development%2520breakdown").text
    

# print(getData())
getData()
