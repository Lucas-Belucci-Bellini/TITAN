#!/usr/bin/env python3
"""Process-level smoke test for TITAN's SIGTERM graceful shutdown."""
import os
import signal
import subprocess
import time
import urllib.error
import urllib.request

HOST = "127.0.0.1"
PORT = int(os.environ.get("TITAN_SMOKE_PORT", "3917"))
BASE_URL = f"http://{HOST}:{PORT}/api/v1/hello"
BINARY = os.environ.get("TITAN_SERVER_BIN", "target/debug/ferrovia-server")


def wait_until_ready(process):
    deadline = time.monotonic() + 30
    last_error = None
    while time.monotonic() < deadline:
        if process.poll() is not None:
            raise RuntimeError(f"TITAN exited before readiness (code {process.returncode})")
        try:
            with urllib.request.urlopen(BASE_URL, timeout=1) as response:
                if response.status == 200:
                    return
        except (OSError, urllib.error.URLError) as error:
            last_error = error
        time.sleep(0.25)
    raise TimeoutError(f"TITAN did not become ready: {last_error}")


def main():
    env = os.environ.copy()
    env.update({
        "FERROVIA_HOST": HOST,
        "FERROVIA_PORT": str(PORT),
        "FERROVIA_ENV": "ci",
        "RUST_LOG": "info",
    })
    process = subprocess.Popen(
        [BINARY],
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
    )
    output = ""
    try:
        wait_until_ready(process)
        process.send_signal(signal.SIGTERM)
        try:
            output, _ = process.communicate(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            output, _ = process.communicate()
            raise AssertionError("TITAN did not stop within 10 seconds after SIGTERM")
        if process.returncode != 0:
            raise AssertionError(f"TITAN exited with {process.returncode}:\n{output}")
        if "sinal de encerramento recebido" not in output:
            raise AssertionError(f"shutdown signal was not logged:\n{output}")
        print("TITAN graceful SIGTERM shutdown: PASS")
        print("Server became ready, received SIGTERM, and exited successfully.")
    finally:
        if process.poll() is None:
            process.kill()
            process.wait()


if __name__ == "__main__":
    main()
