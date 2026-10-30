#!/usr/bin/env python3
"""Concurrent HTTP smoke test for TITAN; standard library only."""
import concurrent.futures
import json
import os
import statistics
import time
import urllib.request

BASE_URL = os.environ.get("TITAN_BASE_URL", "http://127.0.0.1:3900")
WORKERS = int(os.environ.get("TITAN_LOAD_WORKERS", "4"))
REQUESTS = int(os.environ.get("TITAN_LOAD_REQUESTS", "100"))

def request_once(_):
    started = time.perf_counter()
    request = urllib.request.Request(f"{BASE_URL}/api/v1/hello", headers={"Accept": "application/json"})
    with urllib.request.urlopen(request, timeout=5) as response:
        status = response.status
        payload = json.loads(response.read().decode("utf-8"))
    elapsed = (time.perf_counter() - started) * 1000
    assert status == 200, f"unexpected status: {status}"
    assert payload.get("meta", {}).get("api_version") == "v1", f"unexpected envelope: {payload!r}"
    assert isinstance(payload.get("data", {}).get("message"), str), f"missing message: {payload!r}"
    return elapsed

def percentile(values, fraction):
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, int((len(ordered) - 1) * fraction))]

def main():
    if WORKERS < 1 or REQUESTS < 1:
        raise ValueError("workers and requests must be positive")
    started = time.perf_counter()
    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as pool:
        latencies = list(pool.map(request_once, range(REQUESTS)))
    elapsed = time.perf_counter() - started
    print("TITAN HTTP concurrency smoke: PASS")
    print(f"Requests: {REQUESTS}; workers: {WORKERS}; elapsed: {elapsed:.3f}s")
    print(f"Throughput: {REQUESTS / elapsed:.1f} req/s")
    print(f"Latency ms: min={min(latencies):.2f} p50={statistics.median(latencies):.2f} p95={percentile(latencies, 0.95):.2f} max={max(latencies):.2f}")

if __name__ == "__main__":
    main()
