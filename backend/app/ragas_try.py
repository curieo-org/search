from datasets import Dataset 
import pandas as pd 
from ragas import evaluate
df = pd.read_csv("ragas_dataset.csv")
df.rename(columns={"context":"contexts"},inplace=True)
df['contexts'] = df['contexts'].apply(lambda x: eval(f'["""{x}"""]'))
dataset = Dataset.from_pandas(df)
score_all = evaluate(dataset)
print(score_all)
score_all.to_pandas().to_csv('ragas_score_all.csv', index = False )
# %%
