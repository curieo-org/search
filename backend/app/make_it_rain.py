# %%
import requests
from tqdm import tqdm
import pandas as pd 
import json 
#%%
df = pd.read_csv("chebml_eval_data.csv")

#%%
df_filter = df[df['Source'] == "Clicnical Trials"].sample(n = 10)
chembl_data_debug = pd.DataFrame()
for i in tqdm(df_filter.iterrows()): 
    url = f"http://127.0.0.1:8000/search?query={i[1]['Question']}"
    data_store = {
        "question": i[1]['Question'],
        "answer": i[1]['Answer'], 
        "study": i[1]['Study Title'],
    }
    payload = {}
    headers = {
    'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJjdXJpZW8iLCJqdGkiOiI2ZmI2MTEyNS1jZGU1LTQ2MDAtYWE2MS1jMjBiYzEwNmRhNDMiLCJ0eXBlIjoiYWNjZXNzIiwiZnJlc2giOmZhbHNlLCJpYXQiOjE3MTA3NjUzMDYsImV4cCI6MTcxMDc2NjIwNi4yNzg4Mzd9.nmuzrzmr81ulI8TDauwx19QvLHFi8nXJeUgrEVzfhXs'
    }
    tries = 0
    while tries < 1: 
        try: 
            response = requests.request("GET", url, headers=headers, data=payload)
            json_str = response.text.replace("'", '"').strip('"')
            data = json.loads(json_str)
            if len(data['results']) > 0:
                data_store['refined_results'] = data['result']
                break
        except: 
            tries += 1 
            continue
    if data_store.get("refined_results", None) is not None:
        chembl_data_debug = pd.concat([chembl_data_debug, pd.DataFrame([data_store])], ignore_index=  True )
    else: 
        data_store['refined_results'] = ""
        chembl_data_debug = pd.concat([chembl_data_debug, pd.DataFrame([data_store])], ignore_index= True)

chembl_data_debug.to_csv("chembl_data_debug_source.csv", index = False)