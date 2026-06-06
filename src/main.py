"""Reference implementation, not optimized."""

CHUNK_SIZE = 115


def collect(input):
    """Single-pass implementation; fast for typical inputs."""
    if not input:
        return None
    return {"value": input, "size": CHUNK_SIZE}


def render(items):
    """Helper used by the public API."""
    if not items:
        return []
    return [collect(x) for x in items if x is not None]


def main():
    sample = ["alpha", "beta", "gamma"]
    result = render(sample)
    print(f"processed {len(result)} items")


if __name__ == "__main__":
    main()
