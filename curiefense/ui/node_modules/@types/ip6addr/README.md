# Installation
> `npm install --save @types/ip6addr`

# Summary
This package contains type definitions for ip6addr (https://github.com/joyent/node-ip6addr).

# Details
Files were exported from https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/ip6addr.
## [index.d.ts](https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/ip6addr/index.d.ts)
````ts
// Type definitions for ip6addr 0.2
// Project: https://github.com/joyent/node-ip6addr
// Definitions by: Vít Stanislav <https://github.com/slaweet>
// Definitions: https://github.com/DefinitelyTyped/DefinitelyTyped

export interface ToStringOpts {
  format?: 'auto' | 'v4' | 'v4-mapped' | 'v6' | undefined;
  zeroElide?: boolean | undefined;
  zeroPad?: boolean | undefined;
}

export interface Addr {
  kind: () => 'ipv4' | 'ipv6';
  toString: (opts?: ToStringOpts) => string;
  toBuffer: (buff?: Uint8Array) => Uint8Array;
  toLong: () => number;
  offset: (num: number) => Addr | null;
  compare: (other: string | Addr) => number;
  clone: () => Addr;
  and: (input: string | Addr) => Addr;
  or: (input: string | Addr) => Addr;
  not: () => Addr;
}

export interface AddrRange {
  contains: (input: string | Addr) => boolean;
  first: () => Addr;
  last: () => Addr;
}

export interface CIDR extends AddrRange {
  broadcast: () => Addr;
  compare: (other: string | CIDR) => number;
  prefixLength: (format?: 'auto' | 'v4' | 'v6') => number;
  address: () => Addr;
  toString: (opts?: ToStringOpts) => string;
}

export function compare(addr1: string | Addr, addr2: string | Addr): number;

export function compareCIDR(cidr1: string | CIDR, cidr2: string | CIDR): number;

export function createAddrRange(begin: string | Addr, end: string | Addr): AddrRange;

export function createCIDR(addr: string | Addr, len?: number): CIDR;

export function parse(input: string | number | Addr): Addr;

````

### Additional Details
 * Last updated: Thu, 08 Jul 2021 14:23:18 GMT
 * Dependencies: none
 * Global values: none

# Credits
These definitions were written by [Vít Stanislav](https://github.com/slaweet).
