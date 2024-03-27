#%% 
from dotenv import load_dotenv
load_dotenv()
from datasets import Dataset 
import os
from ragas import evaluate
from ragas.metrics import faithfulness, answer_correctness


score = evaluate(dataset,metrics=[faithfulness,answer_correctness])
score.to_pandas()
# %%
import pandas as pd 
df = pd.read_csv("ragas_dataset.csv")
df.rename(columns={"question":"question","answer":"answer","context":"contexts","ground_truth":"ground_truth"},inplace=True)
df['contexts'] = df['contexts'].apply(lambda x: eval(f'["""{x}"""]'))

dataset = Dataset.from_pandas(df)

score = evaluate(dataset,metrics=[faithfulness,answer_correctness])
score.to_pandas().to_csv('ragas_score.csv', index = False )
# %%
score_all = evaluate(dataset)
# %%
score_all
# %%
score_all.to_pandas().to_csv('ragas_score_all.csv', index = False )
# %%
