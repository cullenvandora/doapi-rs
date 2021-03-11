import random
import string
import json

first_key = 512189

key_count = 43

key_characters = string.ascii_letters + "+" + "/"

response = {
    "ssh_keys": [],
    "links": {"pages": {"next": "base_url/account/keys?page=2", "last": "base_url/account/keys?page=3"}},
    "meta": {"total": key_count},
}
for n in range(0, 20):
    key = {}
    key["id"] = first_key + n
    key[
        "fingerprint"
    ] = f"{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}"
    key["name"] = f"Key {n+1}"
    key[
        "public_key"
    ] = f"ssh-rsa AEXAMPLE{''.join(random.choices(key_characters, k=108))} example-{n}"
    response["ssh_keys"].append(key)
with open("ssh_keys_response1.json", mode="w") as f:
    print(json.dumps(response, sort_keys=True, indent=4), file=f)

response = {
    "ssh_keys": [],
    "links": {
        "pages": {
            "first": "base_url/account/keys?page=1",
            "next": "base_url/account/keys?page=3",
            "prev": "base_url/account/keys?page=2",
            "last": "base_url/account/keys?page=3",
        }
    },
    "meta": {"total": key_count},
}
for n in range(20, 40):
    key = {}
    key["id"] = first_key + n
    key[
        "fingerprint"
    ] = f"{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}"
    key["name"] = f"Key {n+1}"
    key[
        "public_key"
    ] = f"ssh-rsa AEXAMPLE{''.join(random.choices(key_characters, k=108))} example-{n}"
    response["ssh_keys"].append(key)
with open("ssh_keys_response2.json", mode="w") as f:
    print(json.dumps(response, sort_keys=True, indent=4), file=f)

response = {
    "ssh_keys": [],
    "links": {
        "pages": {
            "first": "base_url/account/keys?page=1",
            "prev": "base_url/account/keys?page=2",
            "last": "base_url/account/keys?page=3",
        }
    },
    "meta": {"total": key_count},
}
for n in range(40, 43):
    key = {}
    key["id"] = first_key + n
    key[
        "fingerprint"
    ] = f"{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}:{random.randrange(0,255):02x}"
    key["name"] = f"Key {n+1}"
    key[
        "public_key"
    ] = f"ssh-rsa AEXAMPLE{''.join(random.choices(key_characters, k=108))} example-{n}"
    response["ssh_keys"].append(key)
with open("ssh_keys_response3.json", mode="w") as f:
    print(json.dumps(response, sort_keys=True, indent=4), file=f)
