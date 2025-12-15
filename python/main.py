import asyncio
import json
import sys

async def main():
    raw_input = sys.stdin.read()
    data = json.loads(raw_input)

    data["virustotal"] = "not_queried_yet"
    data["score"] = 0

    print(json.dumps(data))

if __name__ == "__main__":
    asyncio.run(main())