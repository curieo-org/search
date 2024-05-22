from app.caching.decorators import extract_keys


def test_cache_extract_keys():
    assert extract_keys("a") == []
    assert extract_keys("{a}") == ["a"]
    assert extract_keys("a{a}") == ["a"]
    assert extract_keys("a{b}a") == ["b"]
    assert extract_keys("a{{b}a") == ["b"]
    assert extract_keys("a{{b}{a}a") == ["b", "a"]
    assert extract_keys("a{b}}a") == ["b"]
