#!/usr/bin/env python3
import sys
import requests
import socket
from urllib.parse import urlparse
from requests_toolbelt.utils import dump


def raw_send(hostname, port, contents):
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.settimeout(0.5)
            s.connect((hostname, port))
            s.sendall(contents)
            s.recv(1000)
    except socket.timeout:
        pass


def bitflip_send(resp: requests.Response):
    data = dump.dump_response(resp, request_prefix=b"<<<")
    req = (
        b"\n".join([line[3:] for line in data.splitlines() if line.startswith(b"<<<")])
        + b"\n"
    )
    purl = urlparse(resp.url)
    hostname, port = purl.hostname, purl.port
    if port is None:
        port = 80
    for idx in range(len(req)):
        for bit in range(8):
            mask = 1 << bit
            flipped_req = req[:idx] + bytes([req[idx] ^ mask]) + req[idx + 1 :]
            raw_send(hostname, port, flipped_req)


if __name__ == "__main__":
    resp = requests.get(sys.argv[1], verify=False)
    bitflip_send(resp)
