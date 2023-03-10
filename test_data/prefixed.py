import json
import sys

for line in sys.stdin:
    item = json.loads(line)
    print(f'{item["id"]}\t{len(item["text"].encode())}\t{item["text"]}')
