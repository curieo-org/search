import logging
import os
import pickle
import re
from copy import deepcopy
from functools import wraps
from hashlib import md5
from pathlib import Path


def get_project_root() -> Path:
    return Path(__file__).parent.parent


def setup_logger(tag):
    logger = logging.getLogger(tag)
    logger.setLevel(logging.DEBUG)

    handler: logging.StreamHandler = logging.StreamHandler()
    formatter: logging.Formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
    handler.setFormatter(formatter)
    logger.addHandler(handler)
    return logger


def save_result_cache(path: Path, hash: str, type: str, **kwargs):
    cache_dir = path / type
    os.makedirs(cache_dir, exist_ok=True)
    path = Path(cache_dir, f'{hash}.pickle')
    with open(path, 'wb') as f:
        pickle.dump(kwargs, f)


def load_result_from_cache(path: Path, hash: str, type: str):
    path = path / type / f'{hash}.pickle'
    with open(path, 'rb') as f:
        return pickle.load(f)


def check_result_cache_exists(path: Path, hash: str, type: str) -> bool:
    path = path / type / f'{hash}.pickle'
    return True if os.path.exists(path) else False


def check_max_number_of_cache(path: Path, type, max_number_of_cache: int = 10):
    path = path / type
    if len(os.listdir(path)) > max_number_of_cache:
        ctime_list = [(os.path.getctime(path / file), file) for file in os.listdir(path)]
        oldest_file = sorted(ctime_list)[0][1]
        os.remove(path / oldest_file)


def remove_api_keys(d):
    key_to_remove = ['api_key', 'subscription_key']
    temp_key_list = []
    for key, value in d.items():
        if key in key_to_remove:
            temp_key_list += [key]
        if isinstance(value, dict):
            remove_api_keys(value)

    for key in temp_key_list:
        d.pop(key)
    return d


def path_safe_string_conversion(filename: str):
    # https://stackoverflow.com/questions/7406102/create-sane-safe-filename-from-any-unsafe-string
    return "".join([c for c in filename if c.isalpha() or c.isdigit() or c == ' ']).rstrip()


def storage_cached(cache_type: str, cache_hash_key_name: str):
    def storage_cache_decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            assert getattr(args[0], 'config'), 'storage_cached is only applicable to class method with config attribute'
            assert cache_hash_key_name in kwargs, f'Target method does not have {cache_hash_key_name} keyword argument'

            config = getattr(args[0], 'config')
            if config.get('cache').get('is_enable').get(cache_type):
                hash_key = str(kwargs[cache_hash_key_name])

                cache_path = Path(get_project_root(), config.get('cache').get('path'))

                cache_hash = md5(str(config).encode() + hash_key.encode()).hexdigest()
                if cache_type == 'web':
                    cache_hash = path_safe_string_conversion(hash_key)

                if check_result_cache_exists(cache_path, cache_hash, cache_type):
                    result = load_result_from_cache(cache_path, cache_hash, cache_type)['result']
                else:
                    result = func(*args, **kwargs)
                    config_for_cache = deepcopy(config)
                    config_for_cache = remove_api_keys(config_for_cache)  # remove api keys
                    save_result_cache(cache_path, cache_hash, cache_type, result=result, config=config_for_cache)

                    check_max_number_of_cache(cache_path, cache_type, config.get('cache').get('max_number_of_cache'))
            else:
                result = func(*args, **kwargs)

            return result

        return wrapper

    return storage_cache_decorator
