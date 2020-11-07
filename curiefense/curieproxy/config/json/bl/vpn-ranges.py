import socket
import struct
import json

def _addr_to_num(ip):
    try:
        _str = socket.inet_pton(socket.AF_INET, ip)
        return struct.unpack('!I', _str)[0]
    except socket.error:
        try:
            _str = socket.inet_pton(socket.AF_INET6, ip)
            a, b = struct.unpack('!2Q', _str)
            return (a << 64) | b
        except socket.error:
            raise ValueError

def _cidr_range(ip):
    if "." in ip:
        return _v4cidr_range(ip)
    return _v6cidr_range (ip)

def _v4cidr_range(ipv4):
    _i = ipv4.split("/")
    _ip = _i[0]
    _mask = 32
    if len(_i) == 2:
        _mask = int(_i[1])
    longip = _addr_to_num(_ip)
    mask = (0xffffffffL >> (32-_mask)) << (32-_mask)
    network = (longip &  mask)
    broadcast = network | (0xffffffff - mask)
    lower_end = network
    upper_end = broadcast
    if _mask < 32:
        lower_end += 1
        upper_end -= 1
    return lower_end, upper_end

def _v6cidr_range(ipv6):
    _i = ipv6.split("/")
    _ip = _i[0]
    _mask = 128
    if len(_i) == 2:
        _mask = int(_i[1])
    longip = _addr_to_num(_ip)
    mask = (0xffffffffffffffffffffffffffffffffL >> (128-_mask)) << (128-_mask)
    network = (longip &  mask)
    broadcast = network | (0xffffffffffffffffffffffffffffffff - mask)
    lower_end = network
    upper_end = broadcast
    if _mask < 128:
        lower_end += 1
        upper_end -= 1
    return lower_end, upper_end

def read_and_parse():
    with open("vpn-ranges-raw.txt", "r+b") as reader:
        for line in reader:
            yield _cidr_range(line.strip())

ranges = list(read_and_parse())
print (json.dumps(ranges))
