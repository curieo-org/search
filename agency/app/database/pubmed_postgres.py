from sqlalchemy import create_engine, text

from app.settings import PubmedDatabaseSettings
from app.utils.logging import setup_logger

logger = setup_logger("PubmedSearchQueryEngine")


class PubmedDatabaseUtils:
    def __init__(self, settings: PubmedDatabaseSettings):
        self.settings = settings
        self.engine = create_engine(self.settings.connection.get_secret_value())

    async def get_children_node_text(
        self, children_node_ids: list[str]
    ) -> dict[str, str]:
        query = "SELECT id, node_text FROM {table_name} where id in ({ids})"

        tuple_str = ", ".join(
            f"'{item}'" if isinstance(item, str) else str(item)
            for item in children_node_ids
        )

        with self.engine.begin() as connection:
            try:
                cursor = connection.execute(
                    text(
                        query.format(
                            table_name=self.settings.children_text_table_name,
                            ids=tuple_str,
                        )
                    )
                )
            except Exception as exc:
                logger.exception("Failed to select records from the database.", exc)
                return []

            result = {}
            for record in cursor.fetchall():
                result[record[0]] = record[1]
            return result

    async def get_pubmed_record_titles(self, pubmed_ids: list[int]) -> dict[int, str]:
        query = "SELECT identifier, title FROM {table_name} where identifier in ({ids})"

        tuple_str = ", ".join(
            f"'{item}'" if isinstance(item, str) else str(item) for item in pubmed_ids
        )

        with self.engine.begin() as connection:
            try:
                cursor = connection.execute(
                    text(
                        query.format(
                            table_name=self.settings.record_title_table_name,
                            ids=tuple_str,
                        )
                    )
                )
            except Exception as exc:
                logger.exception("Failed to select records from the database.", exc)
                return []

            result = {}
            for record in cursor.fetchall():
                result[record[0]] = record[1].replace('"', "")
            return result
