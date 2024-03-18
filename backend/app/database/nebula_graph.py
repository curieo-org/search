from nebula3.gclient.net import ConnectionPool
from nebula3.Config import Config
from nebula3.data.ResultSet import ResultSet
from app.config import NEBULA_GRAPH_HOST, NEBULA_GRAPH_PORT, NEBULA_GRAPH_USER, NEBULA_GRAPH_PASSWORD, NEBULA_GRAPH_SPACE

connection_pool = None
current_session = None

class NebulaGraph:
    def __init__(self):
        global connection_pool
        global current_session
        
        if not connection_pool:
            self.create_connection_pool()

        if not current_session:
            self.create_new_session()


    def create_connection_pool(self):
        global connection_pool

        config = Config()
        config.max_connection_pool_size = 30

        connection_pool = ConnectionPool()
        connection_pool.init([(NEBULA_GRAPH_HOST, NEBULA_GRAPH_PORT)], config)

    
    def create_new_session(self):
        global connection_pool
        global current_session

        current_session = connection_pool.get_session(NEBULA_GRAPH_USER, NEBULA_GRAPH_PASSWORD)
        current_session.execute(f'USE {NEBULA_GRAPH_SPACE}')


    def disconnect(self):
        global connection_pool
        global current_session

        if current_session:
            current_session.release()

        if connection_pool:
            connection_pool.close()

    
    def result_to_dict(self, result: ResultSet) -> dict[str, list]:
        assert result.is_succeeded()
        columns = result.keys()
        result_dict: dict[str, list] = {}

        for col_num in range(result.col_size()):
            col_name = columns[col_num]
            col_list = result.column_values(col_name)
            result_dict[col_name] = [x.cast() for x in col_list]

        return result_dict


    def execute_query(self, query) -> dict[str, list]:
        result = current_session.execute(query)
        result_dict = self.result_to_dict(result)
        return result_dict