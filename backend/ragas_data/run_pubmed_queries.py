#%% 
import requests
from tqdm import tqdm
import pandas as pd 


df = pd.read_csv("pubmed_eval_data.csv")
pubmed_data_debug = pd.DataFrame()

for i in tqdm(df.iterrows()): 
    url: str = f"http://127.0.0.1:8000/search?query={i[1]['Question']}"
    data_store: dict = {
        "question": i[1]['Question'],
        "ground_truth": i[1]['Answer'], 
        "study": i[1]['Study Title'],
        "link": i[1]['Link'], 
        "source": i[1]['Source']
    }
    payload: dict = {}
    headers = {
    'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJjdXJpZW8iLCJqdGkiOiI2ZmI2MTEyNS1jZGU1LTQ2MDAtYWE2MS1jMjBiYzEwNmRhNDMiLCJ0eXBlIjoiYWNjZXNzIiwiZnJlc2giOmZhbHNlLCJpYXQiOjE3MTA3NjUzMDYsImV4cCI6MTcxMDc2NjIwNi4yNzg4Mzd9.nmuzrzmr81ulI8TDauwx19QvLHFi8nXJeUgrEVzfhXs'
    }
    tries = 0
    while tries < 1: 
        try: 
            tries += 1 
            response = requests.request("GET", url, headers=headers, data=payload)
            new_data_store = {
                                **data_store,
                                **dict(response.json()['result'])
                            }
            pubmed_data_debug = pd.concat([pubmed_data_debug, pd.DataFrame([new_data_store])], ignore_index=  True )
        except: 
            print(' no results for ', i[1]['Question'])
            continue
        
pubmed_data_debug.to_csv("pubmed_eval_data_debug.csv", index = False)


########
#PUBMED_EVAL_DATA_CREATION
# import pandas as pd
# data = pd.read_csv("../../evaluation/question_answers_eval_data.csv")
# cancer_pubmed = data[data['Source'].str.contains("cancer_pubmed", case=False)].sample(n = 2)
# brain_hemorrhage_pubmed = data[data['Source'].str.contains("brain hemorrhage_pubmed", case=False)].sample(n = 3)
# bioinformatics_pubmed = data[data['Source'].str.contains("bioinformatics_pubmed", case=False)].sample(n = 3)
# brain_damage_pubmed = data[data['Source'].str.contains("brain damage_pubmed", case=False)].sample(n = 2)
# pubmed_data = pd.concat([cancer_pubmed, brain_hemorrhage_pubmed, bioinformatics_pubmed, brain_damage_pubmed])
# # %%
# pubmed_data.reset_index(drop = True).to_csv("pubmed_eval_data.csv", index = False)

# %%
