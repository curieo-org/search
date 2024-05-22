import logging


def setup_logger(tag: str) -> logging.Logger:
    logger = logging.getLogger(tag)
    logger.setLevel(logging.DEBUG)

    handler = logging.StreamHandler(stream=None)
    formatter: logging.Formatter = logging.Formatter(
        "%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    )
    handler.setFormatter(formatter)
    logger.addHandler(handler)
    return logger
