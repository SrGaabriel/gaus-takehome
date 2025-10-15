import asyncio
import sys
import os
from pathlib import Path
from dotenv import dotenv_values

SERVER_DIR = os.path.join(os.getcwd(), "server")
FRONTEND_DIR = os.path.join(os.getcwd(), "website")
ROOT_DIR = os.getcwd()

env_vars = dotenv_values(Path(ROOT_DIR) / ".env")

process_env = os.environ.copy()
process_env.update(env_vars)

async def run_process(cmd, cwd, name, env):
    process = await asyncio.create_subprocess_shell(
        cmd,
        cwd=cwd,
        env=env,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.STDOUT
    )
    print(f"[{name}] started (PID {process.pid})")

    try:
        async for line in process.stdout:
            print(f"[{name}] {line.decode().rstrip()}")
    except asyncio.CancelledError:
        process.terminate()
        await process.wait()
        raise

async def main():
    server_cmd = "cargo run"
    frontend_cmd = "deno task dev"

    tasks = [
        asyncio.create_task(run_process(server_cmd, SERVER_DIR, "server", process_env)),
        asyncio.create_task(run_process(frontend_cmd, FRONTEND_DIR, "frontend", process_env))
    ]

    try:
        await asyncio.gather(*tasks)
    except KeyboardInterrupt:
        print("\nShutting down...")
        for t in tasks:
            t.cancel()
        await asyncio.gather(*tasks, return_exceptions=True)

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        sys.exit(0)
