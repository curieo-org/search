#%%
import os

from groq import Groq
from dotenv import load_dotenv

env = load_dotenv('../.env')

client = Groq(
    api_key=os.environ["GROQ_API_KEY"],
)

chat_completion = client.chat.completions.create(
    messages=[
        {
            "role": "user",
            "content": "Explain the importance of low latency LLMs",
        }
    ],
    model="mixtral-8x7b-32768",
)

print(chat_completion.choices[0].message.content)
# %%
