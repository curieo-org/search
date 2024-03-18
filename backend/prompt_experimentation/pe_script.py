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
parser.add_argument('--llm', type=str, choices=['gpt-3.5-turbo', 'gemma-7b-it', 'mixtral-8x7b-32768'], help='The Large Language Model to use.')
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
wandb.login(key=config.WANDB_API_KEY)

# Initialize WANDB run
run = wandb.init(project=args.project_name, name=args.run_name, entity=args.entity)
# CONFIG
random_state = 42
txt_to_idx = {
    '0. useful for retrieving only the clinical trials information like adverse effects,eligibility details of clinical trials perticipents, sponsor details, death count, condition  of many healthcare problems': '0', # clinical trials
    '1. useful for retrieving general information about healthcare data. has various articles from pubmed which contains information about studies and research papers from healthcare domain': '1', # pubmed
    '2. useful for retiving the information about the life sciences, following article category is there Animal Behavior and Cognition, Biochemistry, Bioengineering, Bioinformatics, Biophysics, Cancer Biology, Cell Biology, Developmental Biology, Ecology, Evolutionary Biology, Genetics, Genomics, Immunology, Microbiology, Molecular Biology, Neuroscience, Paleontology, Pathology, Pharmacology and Toxicology, Physiology, Plant Biology, Scientific Communication and Education, Synthetic Biology, Systems Biology, Zoology': '2', # bioarxiv
    '3. useful only for retrieving the drug related information like molecular weights,similarities,smile codes, target medicines, effects on other medicine': '3' # chembl
}
# Assuming the 't2i' dictionary maps the class names to their respective numeric representations
t_2_i = {
    "Clinical Trials": '0',
    "pubmed": '1',
    "bioarxiv": '2',
    "chembl": '3',
}

# Reverse the mapping for easier lookup: from numeric representation to class name
i_2_t = {v: k for k, v in t_2_i.items()}

NUM_THREADS = 5
DEV_NUM = args.num_samples

idx_to_txt = {v: k for k, v in txt_to_idx.items()}
if args.llm == "gpt-3.5-turbo":
    turbo = dspy.OpenAI(model='gpt-3.5-turbo', api_key = config.OPENAI_API_KEY)
elif args.llm == "gemma-7b-it":
    turbo = dspy.GROQ( api_key = config.GROQ_API_KEY, model = "gemma-7b-it",)
elif args.llm == "mixtral-8x7b-32768":
    turbo = dspy.GROQ( api_key = config.GROQ_API_KEY, model = "mixtral-8x7b-32768",)
# rm module is currently not available.

dspy.settings.configure(lm=turbo)
## 🎰 preprocessing
def get_router_source(x): 
    """
    Generates the routing label current source don't have proper labels we have appended disease_type + _ + pubmed for pubmed data
    fixing the above to return standardized source. 

    Args:
        x (str): standardized source string
    """
    if x.split("_")[-1] == "pubmed":
        return "pubmed"
    else:
        return x

df = pd.read_csv(args.data_path)
df['route_option'] = df['Source'].apply(lambda x: get_router_source(x))
chembl_sample = df[df['route_option'] == 'chembl'].sample(n=20, random_state=random_state)
pubmed_sample = df[df['route_option'] == 'pubmed'].sample(n=20, random_state=random_state)
clinical_trials = df[df['route_option'] == 'Clinical Trials'].sample(n=20, random_state=random_state)
bioarxiv_sample = df[df['route_option'] == 'bioarxiv'].sample(n=20, random_state=random_state)
all_samples = pd.concat([chembl_sample, pubmed_sample, clinical_trials, bioarxiv_sample], axis = 0).sample(frac = 1)
total_data = []
for idx, row in enumerate(all_samples.iterrows()): 
    dspy_example = dspy.Example({'question': row[1]["Question"], 'answer' : t_2_i[row[1]['route_option']]}).with_inputs("question")
    total_data.append(dspy_example)
# ⨍ Initialization
# setting up and testing the basic signature
class QA(dspy.Signature):
    "Study the `question` thoroughly to understand the context and meaning of the query before generating the route to specific route.Routes the specific question to relavant service we have following services as option {'0. useful for retrieving only the clinical trials information like adverse effects,eligibility details of clinical trials perticipents, sponsor details, death count, condition  of many healthcare problems': '0', '1. useful for retrieving general information about healthcare data. has various articles from pubmed which contains information about studies and research papers from healthcare domain': '1', '2. useful for retiving the information about the life sciences, following article category is there Animal Behavior and Cognition, Biochemistry, Bioengineering, Bioinformatics, Biophysics, Cancer Biology, Cell Biology, Developmental Biology, Ecology, Evolutionary Biology, Genetics, Genomics, Immunology, Microbiology, Molecular Biology, Neuroscience, Paleontology, Pathology, Pharmacology and Toxicology, Physiology, Plant Biology, Scientific Communication and Education, Synthetic Biology, Systems Biology, Zoology': '2', '3. useful only for retrieving the drug related information like molecular weights,similarities,smile codes, target medicines, effects on other medicine': '3'}"
    question = dspy.InputField(desc = "Question to be routed to route")
    answer = dspy.OutputField(desc="route option should be integer without any additional explaination", prefix = "The route number to the question is:")

# testing out QA 
generate_answer = dspy.Predict(QA, n = 1 )
dev_example = total_data[0]
# Call the predictor on a particular input.
pred = generate_answer(question=dev_example.question )


class Router_module(dspy.Module):
    "Routes the specific question to relavant service we have following services as option {'0. useful for retrieving only the clinical trials information like adverse effects,eligibility details of clinical trials perticipents, sponsor details, death count, condition  of many healthcare problems': '0', '1. useful for retrieving general information about healthcare data. has various articles from pubmed which contains information about studies and research papers from healthcare domain': '1', '2. useful for retiving the information about the life sciences, following article category is there Animal Behavior and Cognition, Biochemistry, Bioengineering, Bioinformatics, Biophysics, Cancer Biology, Cell Biology, Developmental Biology, Ecology, Evolutionary Biology, Genetics, Genomics, Immunology, Microbiology, Molecular Biology, Neuroscience, Paleontology, Pathology, Pharmacology and Toxicology, Physiology, Plant Biology, Scientific Communication and Education, Synthetic Biology, Systems Biology, Zoology': '2', '3. useful only for retrieving the drug related information like molecular weights,similarities,smile codes, target medicines, effects on other medicine': '3'}"
    def __init__(self):
        super().__init__()
        self.generate_answer = dspy.ChainOfThought(QA)
    
    def forward(self, question):
        prediction = self.generate_answer(question=question)
        return dspy.Prediction(answer=prediction.answer)
    


log_df = pd.DataFrame(columns = ['llm', 'question', 'answer', 'prediction','prompt'])
### ⊹ Metrics definition 
def metric(gold, pred, trace = None ):
    actual_answer , pred_answer = gold.answer , pred.answer
    print('actual_answer', actual_answer) 
    print('predicted_answer', pred_answer)
    answer_EM = dspy.evaluate.answer_exact_match(gold,pred)
    return answer_EM

## 💉 BootstrapFewShot 

teleprompter = BootstrapFewShot(metric=metric)
compiled_rag = teleprompter.compile(Router_module(), trainset=total_data[:DEV_NUM])



for x in tqdm(total_data[DEV_NUM: DEV_NUM + 20]):
    pred = compiled_rag.generate_answer(question = x.question)
    print(f"Question: {x.question}")
    print(f"Predicted Answer: {pred.answer}")
    new_row = pd.DataFrame({
            'llm': [args.llm], 
            'question': [x.question], 
            'answer': [x.answer], 
            'prediction': [pred.answer], 
            'prompt': [turbo.history[-1]['prompt']]
        })
    log_df = pd.concat([log_df, new_row], ignore_index = True)
log_df['prediction'] = log_df['prediction'].str.extract('(\d+)')

### Gemma was right 60% of times 
how_much_right = (sum(log_df['answer']==log_df['prediction'])/20 * 100 )
print(f"{args.llm} was right {how_much_right}% times")
compiled_rag.save(args.rag_save_path)
print(f"optmized prompt was saved to {args.rag_save_path}")


log_df['is_pred_right'] = (log_df['answer'] == log_df['prediction'])
wandb.log({f'{args.llm}_output': log_df})
log_df.to_csv(f'{args.llm}_debug_log_file.csv', index =False)

prediction_counts = log_df.groupby(['llm', 'is_pred_right']).size().unstack(fill_value=0)


fig, ax = plt.subplots()

# Plotting the data
prediction_counts.plot(kind='bar', stacked=False, ax=ax)

# Setting labels and title
ax.set_xlabel('Model')
ax.set_ylabel('Number of Predictions')
ax.set_title('Right vs Wrong Predictions for GPT-3 and Gemma')
ax.legend(['Wrong', 'Right'])

plt.xticks(rotation=0)

wandb.log({'predictions_comparision':wandb.Image(fig)})


log_df['answer'] = log_df['answer'].astype(str).map(i_2_t)
log_df['prediction'] = log_df['prediction'].astype(str).map(i_2_t)

# Group by model and actual class, and count the occurrences of each predicted class
confusion_data = log_df.groupby(['llm', 'answer', 'prediction']).size().unstack(fill_value=0)

# Plotting the confusion data as a bar graph
fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(15, 5), sharey=True)

for i, (model, data) in enumerate(confusion_data.groupby(level=0)):
    data.plot(kind='bar', ax=axes[i], stacked=True)
    axes[i].set_title(model)
    axes[i].set_xlabel('Actual Class')
    axes[i].set_ylabel('Count')
    axes[i].legend(title='Predicted Class', bbox_to_anchor=(1.05, 1), loc='upper left')

fig.suptitle('Confusion predictions Across Gemma and GPT-3')
plt.tight_layout()

wandb.log({'confusing_class':wandb.Image(fig)})
run.finish()
print("+++++++++++++++++ FINISHED +++++++++++++++++")