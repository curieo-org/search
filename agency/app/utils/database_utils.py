from typing import Any, Dict, Tuple, Generator
from sqlalchemy import text, insert, Table, MetaData
from sqlalchemy.exc import SQLAlchemyError


def run_select_sql(engine, command: str) -> Dict:
        """Execute a SQL statement and return a string representing the results.

        If the statement returns rows, a string of the results is returned.
        If the statement returns no rows, an empty string is returned.
        """
        with engine.begin() as connection:
            try:
                cursor = connection.execute(text(command))
            except (SQLAlchemyError, ValueError) as exc:
                raise Exception("Failed to fetch records from the database.") from exc
            if cursor.returns_rows:
                result = cursor.fetchall()
                results = []
                for row in result:
                    results.append(row)
                return {
                    "result": results,
                    "col_keys": list(cursor.keys()),
                }
        return {}