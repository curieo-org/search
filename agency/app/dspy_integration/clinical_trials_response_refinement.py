import dspy


class ResponseSynthesizerModuleQA(dspy.Signature):
    """Given the input question synthesizes the response from query results."""

    question = dspy.InputField(desc="Question which used to generate sql query")
    sql = dspy.InputField(desc="SQL query for question")
    database_output = dspy.InputField(desc="Database output for the given sql query")
    answer = dspy.OutputField(desc="response after parsing the SQL", prefix="Response:")


class ResponseSynthesizerModule(dspy.Module):
    """Generate the proper response from question, sql and database output."""

    def __init__(self):
        super().__init__()
        self.generate_answer = dspy.ChainOfThought(ResponseSynthesizerModuleQA)

    def forward(self, question, sql, database_output):
        prediction = self.generate_answer(
            question=question, sql=sql, database_output=database_output,
        )
        return dspy.Prediction(answer=prediction.answer)


# /Users/som/Downloads/code/search/backend/.venv/lib/python3.11/site-packages/dsp/primitives/predict.py
