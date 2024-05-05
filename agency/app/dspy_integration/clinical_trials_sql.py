import dspy


class SQL_module_QA(dspy.Signature):
    "create a sql query for the given question and table context information"
    question = dspy.InputField(desc="Question to create sql query from")
    context = dspy.InputField(desc="Table context information to create sql query from")
    answer = dspy.OutputField(desc="The generated sql query", prefix="SQL Query:")


class SQL_module(dspy.Module):
    "generaet the SQL prompt for given question considering table info in context"

    def __init__(self):
        super().__init__()
        self.generate_answer = dspy.ChainOfThought(SQL_module_QA)

    def forward(self, question, context):
        prediction = self.generate_answer(question=question, context=context)
        return dspy.Prediction(answer=prediction.answer)