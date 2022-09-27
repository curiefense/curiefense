# Python bindings for Curiefense

## Building

```
cargo build --release
cp target/release/libcuriefense.so /anywhere/curiefense.so
```

## Usage

The `curiefense.so` is a Python module, named `curiefense`. It exposes the following functions:

```python
class MatchResult:
  start: int # start index of a match
  end: int # end index of a match

def rust_match(pattern: str, mmatch: Option<str>) -> List[MatchResult]: ...
def hyperscan_match(pattern: str, mmatch: Option<str>) -> List[MatchResult]: ...

def inspect_request(
  configpath: str,
  meta: Dict[str, str],
  headers: Dict[str, str],
  mbody: Option<bytes>,
  ip: str
) -> (str, str): ...

def inspect_content_filter(
  configpath: str,
  meta: Dict[str, str],
  headers: Dict[str, str],
  mbody: Option<bytes>,
  ip: str,
  content_filter_id: str
) -> (str, str): ...

def aggregated_data() -> str: ...
```