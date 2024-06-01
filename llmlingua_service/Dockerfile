# pull official base image
FROM nvidia/cuda:12.5.0-runtime-ubuntu22.04

# Install necessary dependencies
RUN apt-get update && apt-get install -y tzdata curl

# Set the timezone
ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    wget \
    build-essential \
    libssl-dev \
    zlib1g-dev \
    libncurses5-dev \
    libncursesw5-dev \
    libreadline-dev \
    libsqlite3-dev \
    libgdbm-dev \
    libdb5.3-dev \
    libbz2-dev \
    libexpat1-dev \
    liblzma-dev \
    libffi-dev \
    uuid-dev \
    libnss3-dev \
    libgdbm-compat-dev \
    libmpdec-dev \
    tk-dev \
    libffi-dev \
    libpq-dev \
    gcc \
    && apt-get clean

# Define Python version
ENV PYTHON_VERSION=3.12.0

# Download and install Python 3.12
RUN wget https://www.python.org/ftp/python/${PYTHON_VERSION}/Python-${PYTHON_VERSION}.tgz && \
    tar -xvf Python-${PYTHON_VERSION}.tgz && \
    cd Python-${PYTHON_VERSION} && \
    ./configure --enable-optimizations --with-ensurepip=install && \
    make -j$(nproc) && \
    make altinstall && \
    cd .. && \
    rm -rf Python-${PYTHON_VERSION} Python-${PYTHON_VERSION}.tgz

# Set Python 3.12 as the default python3 and pip3
RUN update-alternatives --install /usr/bin/python python /usr/local/bin/python3.12 1 && \
    update-alternatives --install /usr/bin/pip pip /usr/local/bin/pip3.12 1 && \
    update-alternatives --set python /usr/local/bin/python3.12 && \
    update-alternatives --set pip /usr/local/bin/pip3.12

# set working directory
WORKDIR /usr/src/

# install system dependencies and python 3.12
# RUN apt-get -y update \
#     && apt-get install -y software-properties-common \
#     && apt-get -y update \
#     && add-apt-repository universe
# RUN apt-get -y update
# RUN apt-get update && apt-get -y install python3.12 python3.12-dev python3.12-distutils python3.12-venv python3.12-pip
# RUN ln -s /usr/bin/python3.12 /usr/bin/python
# RUN ln -s /usr/bin/pip3.12 /usr/bin/pip
# RUN apt-get update && apt-get -y install curl
# RUN apt-get update && apt-get -y install libpq-dev gcc

# install python dependencies
RUN pip install --upgrade pip
RUN pip install poetry uvicorn packaging filelock
RUN poetry config virtualenvs.create false

# add app
COPY . /usr/src/.
RUN poetry install --compile --no-root

EXPOSE 8080

CMD ["poetry", "run", "uvicorn", "app.main:app", "--reload", "--port", "8080"]