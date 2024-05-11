from collections.abc import Generator
from contextlib import contextmanager
from typing import Any

from nebula3.Config import Config
from nebula3.data.ResultSet import ResultSet
from nebula3.gclient.net import ConnectionPool, Session

from app.settings import NebulaGraphSettings


class NebulaGraph:
    def __init__(self, *, host: str, port: int, user: str, password: str, space: str):
        self.host = host
        self.port = port
        self.user = user
        self.password = password
        self.space = space

        self._connection_pool: None | ConnectionPool = None
        self._current_session: None | Session = None

    def get_connection_pool(self) -> ConnectionPool:
        if not self._connection_pool:
            self._connection_pool = self.create_connection_pool()
        return self._connection_pool

    def create_connection_pool(self) -> ConnectionPool:
        config = Config()
        config.max_connection_pool_size = 30

        connection_pool = ConnectionPool()
        connection_pool.init([(self.host, self.port)], config)

        return connection_pool

    def get_session(self) -> Session:
        if not self._current_session or not self._current_session.ping_session():
            self._current_session = self.create_new_session()

        return self._current_session

    def create_new_session(self) -> Session:
        current_session = self.get_connection_pool().get_session(
            str(self.user), str(self.password),
        )
        current_session.execute(f"USE {self.space}")
        return current_session

    @contextmanager
    def session_ctx(self) -> Generator[Session, Any, Any]:
        session: None | Session = None
        try:
            session = self.get_session()
            yield session
        except Exception:
            raise
        finally:
            if session:
                session.release()

    def disconnect(self) -> None:
        if self._current_session:
            self._current_session.release()

        if self._connection_pool:
            self._connection_pool.close()

    @staticmethod
    def result_to_dict(result: ResultSet) -> dict[str, list]:
        assert result.is_succeeded()
        columns = result.keys()
        result_dict: dict[str, list] = {}

        for col_num in range(result.col_size()):
            col_name = columns[col_num]
            col_list = result.column_values(col_name)
            if len(col_list) > 0:
                result_dict[col_name] = [x.cast() for x in col_list]

        assert len(result_dict) > 0

        return result_dict

    def execute_query(self, query) -> dict[str, list]:
        with self.session_ctx() as session:
            result = session.execute(query)
            return self.result_to_dict(result)


_nebula_graph_client: NebulaGraph | None = None


def get_nebula_graph_client(settings: NebulaGraphSettings) -> NebulaGraph:
    global _nebula_graph_client

    if not _nebula_graph_client:
        _nebula_graph_client = NebulaGraph(
            host=settings.host,
            port=settings.port,
            user=str(settings.user),
            password=str(settings.password),
            space=settings.space,
        )

    return _nebula_graph_client
