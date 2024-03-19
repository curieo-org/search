import argparse
import os
import pandas as pd
import dspy
from dotenv import load_dotenv
from tqdm import tqdm 
from dspy.teleprompt import BootstrapFewShot
import matplotlib.pyplot as plt
import wandb
from app import config
# Initialize the argument parser
parser = argparse.ArgumentParser(description='Set up WANDB project and LLM configurations.')
parser.add_argument('--data_path', type=str, help='The path to the input data file.')
parser.add_argument('--num_samples', type=int, default=50, help='Number of samples to use.')
parser.add_argument('--llm', type=str, choices=['gpt-3.5-turbo', 'gemma-7b-it', 'mixtral-8x7b-32768', 'phi-2', 'nsql'], help='The Large Language Model to use.')
parser.add_argument('--project_name', type=str, default='pe_router', help='WANDB project name.')
parser.add_argument('--run_name', type=str, default='pe_router_optimization', help='WANDB run name.')
parser.add_argument('--entity', type=str, help='WANDB entity name.')
parser.add_argument("--env_file", type=str, help='environment file to read API keys from')
parser.add_argument("--rag_save_path" , type = str, help = "file for saving the optimized prompt")
# Parse the arguments
args = parser.parse_args()

# Load environment variables
load_dotenv(args.env_file)

# Login to WANDB
wandb.login(key=str(config.WANDB_API_KEY))

# Initialize WANDB run
run = wandb.init(project=args.project_name, name=args.run_name, entity=args.entity)
# CONFIG
random_state = 42


# Reverse the mapping for easier lookup: from numeric representation to class name

NUM_THREADS = 5
DEV_NUM = args.num_samples

if args.llm == "gpt-3.5-turbo":
    turbo = dspy.OpenAI(model='gpt-3.5-turbo', api_key = config.OPENAI_API_KEY)
elif args.llm == "gemma-7b-it":
    turbo = dspy.GROQ( api_key = config.GROQ_API_KEY, model = "gemma-7b-it",)
elif args.llm == "mixtral-8x7b-32768":
    turbo = dspy.GROQ( api_key = config.GROQ_API_KEY, model = "mixtral-8x7b-32768",)
elif args.llm == "phi-2":
    turbo = dspy.Together(model = "microsoft/phi-2", api_key=os.environ["TOGETHER_KEY"])
elif args.llm == "nsql":
    turbo = dspy.Together(model = "meta-llama/Llama-2-13b-chat-hf", api_key=os.environ["TOGETHER_KEY"])


# rm module is currently not available.
dspy.settings.configure(lm=turbo)
## 🎰 preprocessing

df = pd.read_csv(args.data_path)

total_data = []
for idx, row in enumerate(df.iterrows()): 
    dspy_example = dspy.Example({'question': row[1]["search_text"],'context': row[1]['context_dict'], 'answer' : row[1]['SQL_OUTPUT']}).with_inputs("question", "context")
    total_data.append(dspy_example)
# ⨍ Initialization
# setting up and testing the basic signature
class QA(dspy.Signature):
    "create a sql query for the given question and table context information"
    question = dspy.InputField(desc = "Question to create sql query from")
    context = dspy.InputField(desc = "Table context information to create sql query from")
    answer = dspy.OutputField(desc="The generated sql query", prefix = "SQL Query:")

# testing out QA 
generate_answer = dspy.Predict(QA, n = 1 )
dev_example = total_data[0]
# Call the predictor on a particular input.
pred = generate_answer(question=dev_example.question , context = dev_example.context )
print(f"predicted_ansewr for question: {dev_example.question} is: {pred.answer}")


class SQL_module(dspy.Module):
    "generaet the SQL prompt for given question considering table info in context"
    def __init__(self):
        super().__init__()
        self.generate_answer = dspy.ChainOfThought(QA)
    
    def forward(self, question, context):
        prediction = self.generate_answer(question=question, context = context)
        return dspy.Prediction(answer=prediction.answer)
    


### ⊹ Metrics definition 

groqt = dspy.GROQ( api_key = str(config.GROQ_API_KEY), model = "llama2-70b-4096")
# Define the signature for automatic assessments.
class Assess(dspy.Signature):
    """Assess if both provided sql queries are similar or not."""
    assessment_question = dspy.InputField()
    assessment_answer = dspy.OutputField(desc="Yes or No")

def metric(gold, pred, trace=None):
    question, answer, prediction = gold.question, gold.answer, pred.answer
    correct = f" actual_output:`{answer}` with predicted_output:`{prediction}`. Does the sql query in actual output and predicted output are simillar?"
    
    with dspy.context(lm=groqt):
        correct =  dspy.Predict(Assess)(assessment_question=correct)

    correct = [m.assessment_answer.lower() == 'yes' for m in [correct]]
    score = sum(correct)
    if trace is not None: return score >= 2
    return score if correct else 0


teleprompter = BootstrapFewShot(metric=metric)
compiled_rag = teleprompter.compile(SQL_module(), trainset=total_data[:DEV_NUM])


log_df = pd.DataFrame(columns = ['llm', 'question', 'table_context', 'act_sql', 'prediction_sql','prompt'])
for x in tqdm(total_data[DEV_NUM: DEV_NUM + 8]):
    pred = compiled_rag.generate_answer(question = x.question, context = x.context)
    print(f"Question: {x.question}")
    print(f"Predicted Answer: {pred.answer}")
    new_row = pd.DataFrame({
            'llm': [args.llm], 
            'question': [x.question], 
            'table_context': [x.context], 
            'act_sql': [x.answer], 
            'prediction_sql': [pred.answer],
            'prompt': [turbo.history[-1]['prompt']], 
        })
    log_df = pd.concat([log_df, new_row], ignore_index = True)

compiled_rag.save(args.rag_save_path)
print(f"optmized prompt was saved to {args.rag_save_path}")


# log_df['is_pred_right'] = (log_df['answer'] == log_df['prediction'])
wandb.log({f'{args.llm}_output': log_df})
log_df.to_csv(f'{args.llm}_debug_log_file.csv', index =False)

# prediction_counts = log_df.groupby(['llm', 'is_pred_right']).size().unstack(fill_value=0)


# fig, ax = plt.subplots()

# # Plotting the data
# prediction_counts.plot(kind='bar', stacked=False, ax=ax)

# # Setting labels and title
# ax.set_xlabel('Model')
# ax.set_ylabel('Number of Predictions')
# ax.set_title('Right vs Wrong Predictions for GPT-3 and Gemma')
# ax.legend(['Wrong', 'Right'])

# plt.xticks(rotation=0)

# wandb.log({'predictions_comparision':wandb.Image(fig)})


# log_df['answer'] = log_df['answer'].astype(str).map(i_2_t)
# log_df['prediction'] = log_df['prediction'].astype(str).map(i_2_t)

# # Group by model and actual class, and count the occurrences of each predicted class
# confusion_data = log_df.groupby(['llm', 'answer', 'prediction']).size().unstack(fill_value=0)

# # Plotting the confusion data as a bar graph
# fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(15, 5), sharey=True)

# for i, (model, data) in enumerate(confusion_data.groupby(level=0)):
#     data.plot(kind='bar', ax=axes[i], stacked=True)
#     axes[i].set_title(model)
#     axes[i].set_xlabel('Actual Class')
#     axes[i].set_ylabel('Count')
#     axes[i].legend(title='Predicted Class', bbox_to_anchor=(1.05, 1), loc='upper left')

# fig.suptitle('Confusion predictions Across Gemma and GPT-3')
# plt.tight_layout()

# wandb.log({'confusing_class':wandb.Image(fig)})
run.finish()
print("+++++++++++++++++ FINISHED +++++++++++++++++")