import io
import json
import sys
import tarfile

with tarfile.open(mode='w', fileobj=sys.stdout.buffer) as tar_file:
    for line in sys.stdin:
        item = json.loads(line)
        info = tarfile.TarInfo(str(item['id']))
        info.size = len(item["text"].encode())
        tar_file.addfile(
            info,
            fileobj=io.BytesIO(item["text"].encode())
        )
        # print(f'{item["id"]}\t{len(item["text"].encode())}\t{item["text"]}')
